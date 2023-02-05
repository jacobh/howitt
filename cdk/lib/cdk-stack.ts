import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { RustFunction } from "cargo-lambda-cdk";
import path = require("path");
import {
  Certificate,
  CertificateValidation,
} from "aws-cdk-lib/aws-certificatemanager";
import { HttpApi, HttpMethod, DomainName } from '@aws-cdk/aws-apigatewayv2-alpha'
import { HttpLambdaIntegration } from '@aws-cdk/aws-apigatewayv2-integrations-alpha';

const PROJECT_ROOT_DIR = path.resolve(__dirname, "../..");

console.log(PROJECT_ROOT_DIR);

// import * as sqs from 'aws-cdk-lib/aws-sqs';

export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const certificate = new Certificate(this, "howitt-api.haslehurst.net-cert", {
      domainName: "howitt-api.haslehurst.net",
      validation: CertificateValidation.fromDns(),
    })

    const domainName = new DomainName(this, "howitt-api.haslehurst.net-domain-name", {
      domainName: "howitt-api.haslehurst.net",
      certificate
    })

    const webLambda = new RustFunction(this, "howitt-web-lambda", {
      manifestPath: PROJECT_ROOT_DIR,
    });

    const webLambdaIntegration = new HttpLambdaIntegration("howitt-web-lambda-integration", webLambda);

    const api = new HttpApi(this, "howitt-http-api", {
      // disableExecuteApiEndpoint: true,
      defaultDomainMapping: { domainName }
    });

    api.addRoutes({
      path: '/{proxy+}',
      integration: webLambdaIntegration,
      methods: [HttpMethod.ANY]
    })
  }
}
