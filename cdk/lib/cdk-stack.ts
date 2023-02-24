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
import { Architecture } from "aws-cdk-lib/aws-lambda";
import {
  Table,
  Attribute,
  AttributeType,
  BillingMode,
} from "aws-cdk-lib/aws-dynamodb";
import { Policy, PolicyStatement } from "aws-cdk-lib/aws-iam";
import { NodejsFunction } from "aws-cdk-lib/aws-lambda-nodejs";
import { Duration } from "aws-cdk-lib";
import { Rule, Schedule } from "aws-cdk-lib/aws-events";
import * as targets from "aws-cdk-lib/aws-events-targets";
import {
  CachePolicy,
  CloudFrontWebDistribution,
  Distribution,
  OriginProtocolPolicy,
  ViewerProtocolPolicy,
} from "aws-cdk-lib/aws-cloudfront";
import {
  HttpOrigin,
  LoadBalancerV2Origin,
  OriginGroup,
} from "aws-cdk-lib/aws-cloudfront-origins";
import { ApplicationLoadBalancedFargateService } from "aws-cdk-lib/aws-ecs-patterns";
import { DockerImageAsset } from "aws-cdk-lib/aws-ecr-assets";
import {
  Cluster,
  ContainerImage,
  CpuArchitecture,
  OperatingSystemFamily,
} from "aws-cdk-lib/aws-ecs";

const PROJECT_ROOT_DIR = path.resolve(__dirname, "../..");

console.log(PROJECT_ROOT_DIR);

// import * as sqs from 'aws-cdk-lib/aws-sqs';

export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

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

    const remixRootDir = [PROJECT_ROOT_DIR, "webui"].join("/");

    const remixLambda = new NodejsFunction(this, "remix-webui", {
      architecture: Architecture.ARM_64,
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

    const remixApi = new HttpApi(this, "howitt-webui-api", {
      // disableExecuteApiEndpoint: true,
      defaultDomainMapping: { domainName: webUIDomainName },
    });

    remixApi.addRoutes({
      path: "/{proxy+}",
      integration: remixLambdaIntegration,
      methods: [HttpMethod.ANY],
    });

    const warmerDomains = [webUIDomainName.name, apiDomainName.name];

    const warmerSchedule = new Rule(this, "Rule", {
      schedule: Schedule.expression("cron(0/15 * * * ? *)"),
    });

    const warmerLambda = new RustFunction(this, "howitt-worker-lambda", {
      binaryName: "warmer",
      manifestPath: PROJECT_ROOT_DIR,
      architecture: Architecture.ARM_64,
      bundling: {
        architecture: Architecture.ARM_64,
      },
      memorySize: 128,
      timeout: Duration.seconds(30),
      environment: {
        TARGET_DOMAINS: warmerDomains.join(","),
      },
    });

    warmerSchedule.addTarget(new targets.LambdaFunction(warmerLambda));

    const cluster = new Cluster(this, "howitt-cluster");

    const webuiImage = new DockerImageAsset(this, "webuiDockerImage", {
      directory: path.join(PROJECT_ROOT_DIR, "webui"),
    });

    const webuiContainerImage = ContainerImage.fromDockerImageAsset(webuiImage);

    const webuiService = new ApplicationLoadBalancedFargateService(
      this,
      "webuiALBFargateService",
      {
        memoryLimitMiB: 512,
        desiredCount: 1,
        cpu: 256,
        cluster,
        listenerPort: 80,
        runtimePlatform: {
          operatingSystemFamily: OperatingSystemFamily.LINUX,
          cpuArchitecture: CpuArchitecture.ARM64,
        },
        taskImageOptions: {
          image: webuiContainerImage,
        },
      }
    );

    const origin = new LoadBalancerV2Origin(webuiService.loadBalancer, {
      protocolPolicy: OriginProtocolPolicy.HTTP_ONLY,
    });

    const webuiCloudfront = new Distribution(this, "howitt-webui-cloudfront", {
      defaultBehavior: {
        origin,
        viewerProtocolPolicy: ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
        cachePolicy: CachePolicy.CACHING_DISABLED
      },
      additionalBehaviors: {
        'build/*': {
          origin,
          viewerProtocolPolicy: ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
          cachePolicy: CachePolicy.CACHING_OPTIMIZED
        }
      }
    });
  }
}
