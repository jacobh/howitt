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

    // // ---
    // // ECS
    // // ---

    // const cluster = new Cluster(this, "howitt-cluster", { vpc });

    // // Or add customized capacity. Be sure to start the Amazon ECS-optimized AMI.
    // const autoScalingGroup = new AutoScalingGroup(this, "ASG", {
    //   vpc,
    //   launchTemplate: new LaunchTemplate(this, "howitt-cluster-arm-template", {
    //     machineImage: EcsOptimizedImage.amazonLinux2023(AmiHardwareType.ARM),
    //     instanceType: new InstanceType("t4g.small"),
    //     userData: UserData.forLinux(),
    //     role: new Role(this, "howitt-cluster-role", {
    //       assumedBy: new ServicePrincipal("ec2.amazonaws.com"),
    //     }),
    //   }),
    //   minCapacity: 1,
    //   maxCapacity: 1,
    // });

    // const capacityProvider = new AsgCapacityProvider(
    //   this,
    //   "AsgCapacityProvider",
    //   {
    //     autoScalingGroup,
    //   }
    // );
    // cluster.addAsgCapacityProvider(capacityProvider);

    // const howittApiWebImage = new DockerImageAsset(
    //   this,
    //   "howitt-api-web-image",
    //   {
    //     directory: path.join(__dirname, "../.."),
    //     file: "howitt-web.dockerfile",
    //     platform: Platform.LINUX_ARM64,
    //   }
    // );

    // const howittApiTaskDef = new Ec2TaskDefinition(
    //   this,
    //   "howitt-api-web-task-def",
    //   {}
    // );

    // howittApiTaskDef.addContainer("howitt-api-web-container", {
    //   image: ContainerImage.fromDockerImageAsset(howittApiWebImage),
    //   memoryLimitMiB: 256,
    // });

    // new Ec2Service(this, "howitt-api-web-service", {
    //   cluster,
    //   taskDefinition: howittApiTaskDef,
    // });

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

    const apiDomainName = new DomainName(
      this,
      "howitt-api.haslehurst.net-domain-name",
      {
        domainName: "howitt-api.haslehurst.net",
        certificate: new Certificate(this, "howitt-api.haslehurst.net-cert", {
          domainName: "howitt-api.haslehurst.net",
          validation: CertificateValidation.fromDns(),
        }),
      }
    );

    const webUIDomainName = new DomainName(
      this,
      "howitt.haslehurst.net-domain-name",
      {
        domainName: "howitt.haslehurst.net",
        certificate: new Certificate(this, "howitt.haslehurst.net-cert", {
          domainName: "howitt.haslehurst.net",
          validation: CertificateValidation.fromDns(),
        }),
      }
    );

    const webLambda = new RustFunction(this, "howitt-web-lambda", {
      manifestPath: PROJECT_ROOT_DIR,
      binaryName: "howitt-web-lambda",
      architecture: Architecture.ARM_64,
      bundling: {
        architecture: Architecture.ARM_64,
      },
      memorySize: 512,
      environment: {
        HOWITT_TABLE_NAME: dynamoTable.tableName,
      },
    });

    webLambda.addToRolePolicy(
      new PolicyStatement({
        actions: ["dynamodb:*"],
        resources: [dynamoTable.tableArn, `${dynamoTable.tableArn}/index/*`],
      })
    );

    const webLambdaIntegration = new HttpLambdaIntegration(
      "howitt-web-lambda-integration",
      webLambda
    );

    const api = new HttpApi(this, "howitt-http-api", {
      // disableExecuteApiEndpoint: true,
      defaultDomainMapping: { domainName: apiDomainName },
    });

    api.addRoutes({
      path: "/{proxy+}",
      integration: webLambdaIntegration,
      methods: [HttpMethod.ANY],
    });

    const webuiApi = new webuiLambdaDeployment(this);

    // taken from deployed webuiApi
    const origin = new HttpOrigin(
      "6qrdtonstb.execute-api.ap-southeast-2.amazonaws.com"
    );

    const webuiCloudfront = new Distribution(this, "howitt-webui-cloudfront", {
      domainNames: [webUIDomainName.name],
      certificate: Certificate.fromCertificateArn(
        this,
        "howitt-webui-cloudfront-cert",
        "arn:aws:acm:us-east-1:176170034926:certificate/4cbfff19-ce4b-4653-bd7c-3a4167092fe6"
      ),
      httpVersion: HttpVersion.HTTP2_AND_3,
      defaultBehavior: {
        origin,
        viewerProtocolPolicy: ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
        cachePolicy: CachePolicy.CACHING_DISABLED,
      },
      additionalBehaviors: {
        "build/*": {
          origin,
          viewerProtocolPolicy: ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
          cachePolicy: CachePolicy.CACHING_OPTIMIZED,
        },
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

// not in use
class webuiLambdaDeployment extends Construct {
  public httpApi: HttpApi;

  constructor(scope: Construct) {
    super(scope, "webui-lambda-deployment");

    const remixRootDir = [PROJECT_ROOT_DIR, "webui"].join("/");

    const remixLambda = new NodejsFunction(this, "remix-webui", {
      architecture: Architecture.ARM_64,
      runtime: Runtime.NODEJS_18_X,
      memorySize: 1024,
      timeout: Duration.seconds(10),

      bundling: {
        commandHooks: {
          beforeInstall: () => [],
          beforeBundling: (inputDir, outputDir) => [
            `cd ${remixRootDir} && npm run build && cp -R ${remixRootDir}/public ${outputDir}`,
          ],
          afterBundling: (_, outputDir) => [],
        },
      },

      handler: "handler",
      entry: [remixRootDir, "lambdaExpressServer.ts"].join("/"),
    });

    const remixLambdaIntegration = new HttpLambdaIntegration(
      "howitt-remix-lambda-integration",
      remixLambda
    );

    this.httpApi = new HttpApi(this, "howitt-webui-api", {
      // disableExecuteApiEndpoint: true,
      createDefaultStage: true,
      // defaultDomainMapping: { domainName },
    });

    this.httpApi.addRoutes({
      path: "/{proxy+}",
      integration: remixLambdaIntegration,
      methods: [HttpMethod.ANY],
    });
  }
}
