use lazy_static::lazy_static;

lazy_static! {
    pub static ref AWS_PATTERNS: Vec<String> = vec![
        // IAM Resources
        r"arn:aws:iam::\d{12}:role/[A-Za-z0-9_-]+".to_string(),
        r"arn:aws:iam::\d{12}:user/[A-Za-z0-9_-]+".to_string(),
        r"arn:aws:iam::\d{12}:policy/[A-Za-z0-9_-]+".to_string(),
        r"arn:aws:iam::\d{12}:instance-profile/[A-Za-z0-9_-]+".to_string(),
        r"arn:aws:iam::\d{12}:saml-provider/[A-Za-z0-9_-]+".to_string(),
        r"arn:aws:iam::\d{12}:mfa/[A-Za-z0-9_-]+".to_string(),
        r"arn:aws:iam::\d{12}:server-certificate/[A-Za-z0-9_-]+".to_string(),

        // Cloudfront
        r"arn:aws:cloudfront::[0-9]{12}:distribution/[A-Z0-9]+".to_string(),
        r"arn:aws:cloudfront::[0-9]{12}:streaming-distribution/[A-Z0-9]+".to_string(),
        r"arn:aws:cloudfront::[0-9]{12}:origin-access-identity/[A-Z0-9]+".to_string(),
        r"arn:aws:cloudfront::[0-9]{12}:origin-request-policy/[a-f0-9-]{36}".to_string(),
        r"arn:aws:cloudfront::[0-9]{12}:cache-policy/[a-f0-9-]{36}".to_string(),
        r"arn:aws:cloudfront::[0-9]{12}:function/[A-Z0-9]+".to_string(),
        r"[A-Z0-9]{13,14}\.cloudfront\.net".to_string(),

        // EC2 Resources
        r"arn:aws:ec2:[a-z0-9-]+:\d{12}:instance/[a-z0-9]+".to_string(),
        r"arn:aws:ec2:[a-z0-9-]+:\d{12}:security-group/[a-z0-9]+".to_string(),
        r"arn:aws:ec2:[a-z0-9-]+:\d{12}:vpc/[a-z0-9]+".to_string(),
        r"arn:aws:ec2:[a-z0-9-]+:\d{12}:subnet/[a-z0-9]+".to_string(),
        r"arn:aws:ec2:[a-z0-9-]+:\d{12}:volume/[a-z0-9]+".to_string(),
        r"arn:aws:ec2:[a-z0-9-]+:\d{12}:snapshot/[a-z0-9]+".to_string(),
        r"arn:aws:ec2:[a-z0-9-]+:\d{12}:network-interface/[a-z0-9]+".to_string(),
        r"arn:aws:ec2:[a-z0-9-]+:\d{12}:placement-group/[a-zA-Z0-9-]+".to_string(),

        // Storage
        r"arn:aws:s3:::[a-z0-9.-]{3,63}".to_string(),
        r"arn:aws:s3:::[a-z0-9.-]{3,63}/[^*]*".to_string(),
        r"arn:aws:efs:[a-z0-9-]+:\d{12}:file-system/fs-[a-f0-9]{8,}".to_string(),

        // Database
        r"arn:aws:dynamodb:[a-z0-9-]+:\d{12}:table/[a-zA-Z0-9-_]+".to_string(),
        r"arn:aws:rds:[a-z0-9-]+:\d{12}:db/[a-zA-Z0-9-]+".to_string(),
        r"arn:aws:rds:[a-z0-9-]+:\d{12}:cluster/[a-zA-Z0-9-]+".to_string(),
        r"arn:aws:redshift:[a-z0-9-]+:\d{12}:cluster:[a-zA-Z0-9-]+".to_string(),

        // Serverless
        r"arn:aws:lambda:[a-z0-9-]+:\d{12}:function:[a-zA-Z0-9-_]+".to_string(),
        r"arn:aws:lambda:[a-z0-9-]+:\d{12}:function:[a-zA-Z0-9-_]+:[0-9]+".to_string(),
        r"arn:aws:lambda:[a-z0-9-]+:\d{12}:function:[a-zA-Z0-9-_]+:\$LATEST".to_string(),
        r"arn:aws:lambda:[a-z0-9-]+:\d{12}:layer:[a-zA-Z0-9-_]+".to_string(),
        r"arn:aws:lambda:[a-z0-9-]+:\d{12}:layer:[a-zA-Z0-9-_]+:[0-9]+".to_string(),
        r"arn:aws:lambda:[a-z0-9-]+:\d{12}:event-source-mapping/[a-f0-9-]{36}".to_string(),
        r"arn:aws:lambda:[a-z0-9-]+:\d{12}:code-signing-config:[a-zA-Z0-9-_]+".to_string(),
        r"arn:aws:apigateway:[a-z0-9-]+::apis/[a-z0-9]+".to_string(),

        // ECR
        r"arn:aws:ecr:[a-z0-9-]+:\d{12}:repository/[a-zA-Z0-9_-]+".to_string(),
        r"arn:aws:ecr:[a-z0-9-]+:\d{12}:repository/[a-zA-Z0-9_-]+/[a-zA-Z0-9_-]+".to_string(),
        r"\d{12}\.dkr\.ecr\.[a-z0-9-]+\.amazonaws\.com/[a-zA-Z0-9_-]+".to_string(),
        r"\d{12}\.dkr\.ecr\.[a-z0-9-]+\.amazonaws\.com/[a-zA-Z0-9_-]+:[a-zA-Z0-9_.-]+".to_string(),
        r"\d{12}\.dkr\.ecr\.[a-z0-9-]+\.amazonaws\.com/[a-zA-Z0-9_-]+@sha256:[a-f0-9]{64}".to_string(),
        r"arn:aws:ecr-public::[0-9]{12}:repository/[a-zA-Z0-9_-]+".to_string(),
        r"public\.ecr\.aws/[a-z0-9]+/[a-zA-Z0-9_-]+".to_string(),

        // Networking
        r"arn:aws:elasticloadbalancing:[a-z0-9-]+:\d{12}:loadbalancer/[a-zA-Z0-9-]+/[0-9a-f]{8,}".to_string(),
        r"arn:aws:acm:[a-z0-9-]+:\d{12}:certificate/[0-9a-f-]{36}".to_string(),
        r"arn:aws:cloudfront::[0-9]{12}:distribution/[A-Z0-9]+".to_string(),
        r"arn:aws:route53:::hostedzone/[A-Z0-9]+".to_string(),
        r"arn:aws:wafv2:[a-z0-9-]+:\d{12}:regional/webacl/[a-zA-Z0-9-_]+/[a-f0-9-]+".to_string(),

        // Security
        r"arn:aws:kms:[a-z0-9-]+:\d{12}:key/[0-9a-f-]{36}".to_string(),
        r"arn:aws:secretsmanager:[a-z0-9-]+:\d{12}:secret:[A-Za-z0-9/_+=.@-]+".to_string(),
        r"arn:aws:ssm:[a-z0-9-]+:\d{12}:parameter/[a-zA-Z0-9/_.-]+".to_string(),
        r"arn:aws:acm:[a-z0-9-]+:\d{12}:certificate/[0-9a-f-]{36}".to_string(),

        // Monitoring
        r"arn:aws:cloudwatch:[a-z0-9-]+:\d{12}:alarm:[a-zA-Z0-9-_]+".to_string(),
        r"arn:aws:logs:[a-z0-9-]+:\d{12}:log-group:/[a-zA-Z0-9/_.-]+".to_string(),

        // Containers
        r"arn:aws:ecs:[a-z0-9-]+:\d{12}:cluster/[a-zA-Z0-9_-]+".to_string(),
        r"arn:aws:ecs:[a-z0-9-]+:\d{12}:task-definition/[a-zA-Z0-9_-]+:[0-9]+".to_string(),
        r"arn:aws:ecr:[a-z0-9-]+:\d{12}:repository/[a-zA-Z0-9_-]+".to_string(),

        // Messaging
        r"arn:aws:sns:[a-z0-9-]+:\d{12}:[a-zA-Z0-9-_]+".to_string(),
        r"arn:aws:sqs:[a-z0-9-]+:\d{12}:[a-zA-Z0-9-_]+".to_string(),
        r"arn:aws:events:[a-z0-9-]+:\d{12}:rule/[a-zA-Z0-9-_]+".to_string(),
    ];
}
