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
      indexName: 'gsi1',
      partitionKey: { name: "gsi1pk", type: AttributeType.STRING },
      sortKey: { name: "gsi1sk", type: AttributeType.STRING },
    })

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

    const remixRootDir = [PROJECT_ROOT_DIR, 'webui'].join('/')

    const remixLambda = new NodejsFunction(this, "remix-webui", {
      architecture: Architecture.ARM_64,
      memorySize: 1024,
      timeout: Duration.seconds(10),

      bundling: {
        commandHooks: {
          beforeInstall: () => [],
          beforeBundling: (inputDir, outputDir) => [`cd ${remixRootDir} && npm run build && cp -R ${remixRootDir}/public ${outputDir}`],
          afterBundling: (_, outputDir) => [],
        }
      },

      handler: 'handler',
      entry: [remixRootDir, 'lambdaExpressServer.ts'].join('/')
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
  }
}
