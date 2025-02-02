import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { Bucket, StorageClass } from "aws-cdk-lib/aws-s3";

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
