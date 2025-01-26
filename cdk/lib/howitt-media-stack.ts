import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { Bucket } from "aws-cdk-lib/aws-s3";
import * as cloudfront from "aws-cdk-lib/aws-cloudfront";
import * as origins from "aws-cdk-lib/aws-cloudfront-origins";
import * as acm from "aws-cdk-lib/aws-certificatemanager";
import * as s3origins from "aws-cdk-lib/aws-cloudfront-origins";

export class MediaStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const mediaBucket = new Bucket(this, "howitt-media", {
      bucketName: "howitt-media",
      publicReadAccess: true,
      blockPublicAccess: {
        blockPublicAcls: false,
        blockPublicPolicy: false,
        ignorePublicAcls: false,
        restrictPublicBuckets: false,
      },
    });

    const certificate = new acm.Certificate(this, "MediaCertificate", {
      domainName: "media.howittplains.net",
      validation: acm.CertificateValidation.fromDns(),
    });

    const distribution = new cloudfront.Distribution(
      this,
      "MediaDistribution",
      {
        defaultBehavior: {
          origin: new s3origins.S3Origin(mediaBucket),
          viewerProtocolPolicy:
            cloudfront.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
        },
        domainNames: ["media.howittplains.net"],
        certificate: certificate,
      }
    );
  }
}
