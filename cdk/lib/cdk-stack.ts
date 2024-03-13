import * as path from "node:path";
import * as cdk from "aws-cdk-lib";
import { aws_events_targets } from "aws-cdk-lib";
import * as events from "aws-cdk-lib/aws-events";
import * as ssm from "aws-cdk-lib/aws-ssm";
import { RustFunction } from "cargo-lambda-cdk";
import type { Construct } from "constructs";

export class NotificationStack extends cdk.Stack {
	constructor(scope: Construct, id: string, props?: cdk.StackProps) {
		super(scope, id, props);

		const uri = ssm.StringParameter.valueForStringParameter(
			this,
			"/nishiki/url/error-notification/discord",
		);

		const notificationFunction = new RustFunction(
			this,
			"NotificationFunction",
			{
				functionName: "nishiki-discord-notification-function",
				manifestPath: path.join(
					__dirname,
					"../../lambdas/notification-function/Cargo.toml",
				),
				environment: {
					URI: uri,
				},
			},
		);

		const notificationEventBus = new events.EventBus(
			this,
			"NotificationEventBus",
			{
				eventBusName: "NishikiNotificationBus",
			},
		);

		new events.Rule(this, "ErrorEventRule", {
			eventPattern: {
				detailType: ["ErrorNotification"],
			},
			eventBus: notificationEventBus,
			targets: [new aws_events_targets.LambdaFunction(notificationFunction)],
		});
	}
}
