import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { RustFunction } from "cargo-lambda-cdk";
import path = require("path");
import {
  Certificate,
  CertificateValidation,
} from "aws-cdk-lib/aws-certificatemanager";
import {
  HttpApi,
  HttpMethod,
  DomainName,
} from "@aws-cdk/aws-apigatewayv2-alpha";
import { HttpLambdaIntegration } from "@aws-cdk/aws-apigatewayv2-integrations-alpha";
import { Architecture, Code, Runtime } from "aws-cdk-lib/aws-lambda";
import {
  Table,
  Attribute,
  AttributeType,
  BillingMode,
} from "aws-cdk-lib/aws-dynamodb";
import {
  Policy,
  PolicyStatement,
  Role,
  ServicePrincipal,
} from "aws-cdk-lib/aws-iam";
import { NodejsFunction } from "aws-cdk-lib/aws-lambda-nodejs";
import { Duration } from "aws-cdk-lib";
import { Rule, Schedule } from "aws-cdk-lib/aws-events";
import * as targets from "aws-cdk-lib/aws-events-targets";
import {
  CachePolicy,
  CacheQueryStringBehavior,
  CloudFrontWebDistribution,
  Distribution,
  HttpVersion,
  LambdaEdgeEventType,
  OriginProtocolPolicy,
  ViewerProtocolPolicy,
} from "aws-cdk-lib/aws-cloudfront";
import { HttpOrigin } from "aws-cdk-lib/aws-cloudfront-origins";
import { experimental } from "aws-cdk-lib/aws-cloudfront";
import { Bucket } from "aws-cdk-lib/aws-s3";
import {
  AmiHardwareType,
  AsgCapacityProvider,
  Cluster,
  ContainerImage,
  Ec2Service,
  Ec2TaskDefinition,
  EcsOptimizedImage,
  TaskDefinition,
} from "aws-cdk-lib/aws-ecs";
import {
  InstanceType,
  LaunchTemplate,
  UserData,
  Vpc,
} from "aws-cdk-lib/aws-ec2";
import { AutoScalingGroup } from "aws-cdk-lib/aws-autoscaling";
import { DockerImageAsset, Platform } from "aws-cdk-lib/aws-ecr-assets";

const PROJECT_ROOT_DIR = path.resolve(__dirname, "../..");

console.log(PROJECT_ROOT_DIR);

// import * as sqs from 'aws-cdk-lib/aws-sqs';

export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const vpc = Vpc.fromLookup(this, "Vpc", {
      isDefault: true,
    });

    // ---
    // Dynamo
    // ---

    const dynamoTable = new Table(this, "howitt-dynamodb", {
      tableName: "howitt",
      partitionKey: { name: "pk", type: AttributeType.STRING },
      sortKey: { name: "sk", type: AttributeType.STRING },
      billingMode: BillingMode.PAY_PER_REQUEST,
    });

    dynamoTable.addGlobalSecondaryIndex({
      indexName: "gsi1",
      partitionKey: { name: "gsi1pk", type: AttributeType.STRING },
      sortKey: { name: "gsi1sk", type: AttributeType.STRING },
    });

    // ---
    // S3
    // ---

    const photosBucket = new Bucket(this, "howitt-photos", {
      bucketName: "howitt-photos",
      publicReadAccess: true,
      blockPublicAccess: {
        blockPublicAcls: false,
        blockPublicPolicy: false,
        ignorePublicAcls: false,
        restrictPublicBuckets: false,
      },
    });

    const cacheControlFn = new experimental.EdgeFunction(
      this,
      "tiles-cf-cache-control",
      {
        runtime: Runtime.NODEJS_14_X,
        handler: "index.handler",
        code: Code.fromInline(`
        exports.handler = function(event, context, callback) {
          const response = event.Records[0].cf.response;
          const headers = response.headers;

          headers['cache-control'] = [{key: 'Cache-Control', value: 'public, max-age=604800'}];

          callback(null, response);
        }
      `),
      }
    );

    const tileCloudfront = new Distribution(this, "howitt-tiles-cloudfront", {
      httpVersion: HttpVersion.HTTP2_AND_3,
      defaultBehavior: {
        origin: new HttpOrigin("tile.thunderforest.com", {}),
        viewerProtocolPolicy: ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
        cachePolicy: new CachePolicy(this, "tiles-cachepolicy", {
          minTtl: Duration.seconds(1),
          maxTtl: Duration.days(365),
          defaultTtl: Duration.days(1),
          queryStringBehavior: CacheQueryStringBehavior.allowList("apikey"),
        }),
        edgeLambdas: [
          {
            functionVersion: cacheControlFn.currentVersion,
            eventType: LambdaEdgeEventType.ORIGIN_RESPONSE,
          },
        ],
      },
    });
  }
}
