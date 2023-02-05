import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { RustFunction } from "cargo-lambda-cdk";
import path = require("path");
import { LambdaIntegration, RestApi } from "aws-cdk-lib/aws-apigateway";
import {
  Certificate,
  CertificateValidation,
} from "aws-cdk-lib/aws-certificatemanager";

const PROJECT_ROOT_DIR = path.resolve(__dirname, "../..");

console.log(PROJECT_ROOT_DIR);

// import * as sqs from 'aws-cdk-lib/aws-sqs';

export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // The code that defines your stack goes here

    // example resource
    // const queue = new sqs.Queue(this, 'CdkQueue', {
    //   visibilityTimeout: cdk.Duration.seconds(300)
    // });
    const certificate = new Certificate(this, "howitt-api.haslehurst.net", {
      domainName: "howitt-api.haslehurst.net",
      validation: CertificateValidation.fromDns(),
    })

    const webLambda = new RustFunction(this, "howitt-web-lambda", {
      manifestPath: PROJECT_ROOT_DIR,
    });

    const webLambdaIntegration = new LambdaIntegration(webLambda);

    const api = new RestApi(this, "howitt-api", {
      restApiName: "Howitt API",
      disableExecuteApiEndpoint: true,
      domainName: {
        domainName: "howitt-api.haslehurst.net",
        certificate,
      },
    });

    api.root.addProxy({ defaultIntegration: webLambdaIntegration });
  }
}
