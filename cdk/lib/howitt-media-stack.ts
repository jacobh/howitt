import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { Bucket, StorageClass } from "aws-cdk-lib/aws-s3";
import * as cloudfront from "aws-cdk-lib/aws-cloudfront";
import * as origins from "aws-cdk-lib/aws-cloudfront-origins";

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
      lifecycleRules: [
        {
          enabled: true,
          transitions: [
            {
              storageClass: StorageClass.INTELLIGENT_TIERING,
              transitionAfter: cdk.Duration.days(0),
            },
          ],
        },
      ],
    });

    // Create CloudFront distribution
    const distribution = new cloudfront.Distribution(
      this,
      "MediaDistribution",
      {
        defaultBehavior: {
          origin: origins.S3BucketOrigin.withOriginAccessControl(mediaBucket),
          viewerProtocolPolicy:
            cloudfront.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
          allowedMethods: cloudfront.AllowedMethods.ALLOW_GET_HEAD,
          cachedMethods: cloudfront.CachedMethods.CACHE_GET_HEAD,
          cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
        },
        priceClass: cloudfront.PriceClass.PRICE_CLASS_ALL,
        minimumProtocolVersion: cloudfront.SecurityPolicyProtocol.TLS_V1_2_2021,
      }
    );

    new cdk.CfnOutput(this, "MediaDistributionDomainName", {
      value: distribution.distributionDomainName,
      description: "Media CloudFront Distribution Domain Name",
    });

    const backupsBucket = new Bucket(this, "howitt-backups", {
      bucketName: "howitt-backups",
      lifecycleRules: [
        {
          id: "PostgreSQL Daily Backups",
          enabled: true,
          prefix: "postgresql/daily/",
          expiration: cdk.Duration.days(30),
        },
        {
          id: "PostgreSQL Weekly Backups",
          enabled: true,
          prefix: "postgresql/weekly/",
          expiration: cdk.Duration.days(180),
        },
        {
          id: "PostgreSQL Monthly Backups",
          enabled: true,
          prefix: "postgresql/monthly/",
          transitions: [
            {
              storageClass: StorageClass.GLACIER,
              transitionAfter: cdk.Duration.days(1),
            },
          ],
        },
      ],
    });
  }
}
