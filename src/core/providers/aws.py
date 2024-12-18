from typing import Dict, List, Optional

from .base import BaseProvider

class AWSProvider(BaseProvider):
    """
    AWS Provider for managing AWS credentials.

    This provider supplies AWS environment variables and sensitive data for 
    authentication with AWS services.

    It requires AWS credentials, specifically:
    - AWS_ACCESS_KEY_ID
    - AWS_SECRET_ACCESS_KEY

    Optionally, it supports:
    - AWS_SESSION_TOKEN
    """

    def __init__(
        self,
        access_key_id: str,
        secret_access_key: str,
        session_token: Optional[str]=None
    ):
        """
        Initialize the AWSProvider with credentials.

        Args:
            access_key_id (str): AWS access key ID.
            secret_access_key (str): AWS secret access key.
            session_token (Optional[str]): Optional session token for temporary credentials.
        """
        self.credentials = {
            "AWS_ACCESS_KEY_ID": access_key_id,
            "AWS_SECRET_ACCESS_KEY": secret_access_key
        }
        if session_token:
            self.credentials["AWS_SESSION_TOKEN"] = session_token

    def get_environment(self) -> Dict[str, str]:
        """
        Return environment variables for AWS credentials.

        Returns:
            Dict[str, str]: A dictionary with AWS credentials for the environment.
        """
        environment, _ = self._generate_env_and_sensitive(self.credentials)
        return environment

    def get_sensitive(self) -> Dict[str, str]:
        """
        Return sensitive values for masking.

        Returns:
            Dict[str, str]: A dictionary with sensitive AWS credentials.
        """
        _, sensitive = self._generate_env_and_sensitive(self.credentials)
        return sensitive

    def validate(self) -> None:
        """
        Validate that required AWS credentials are provided.

        Raises:
            ValueError: If AWS credentials are incomplete or missing.
        """
        if not self.credentials.get("AWS_ACCESS_KEY_ID") or not self.credentials.get("AWS_SECRET_ACCESS_KEY"):
            raise ValueError("AWS credentials are incomplete.")

    def get_predefined_masked_objects(self) -> List[str]:
        """
        Retrieve a list of predefined sensitive AWS objects that should be masked.

        Returns:
            List[str]: A list of strings representing sensitive AWS objects to be masked.
        """
        return [
            # IAM Resources
            r"arn:aws:iam::\d{12}:role/[A-Za-z0-9_-]+",
            r"arn:aws:iam::\d{12}:user/[A-Za-z0-9_-]+",
            r"arn:aws:iam::\d{12}:policy/[A-Za-z0-9_-]+",
            r"arn:aws:iam::\d{12}:instance-profile/[A-Za-z0-9_-]+",
            r"arn:aws:iam::\d{12}:saml-provider/[A-Za-z0-9_-]+",
            r"arn:aws:iam::\d{12}:mfa/[A-Za-z0-9_-]+",
            r"arn:aws:iam::\d{12}:server-certificate/[A-Za-z0-9_-]+",

            # Cloudfront
            r"arn:aws:cloudfront::[0-9]{12}:distribution/[A-Z0-9]+",
            r"arn:aws:cloudfront::[0-9]{12}:streaming-distribution/[A-Z0-9]+",
            r"arn:aws:cloudfront::[0-9]{12}:origin-access-identity/[A-Z0-9]+",
            r"arn:aws:cloudfront::[0-9]{12}:origin-request-policy/[a-f0-9-]{36}",
            r"arn:aws:cloudfront::[0-9]{12}:cache-policy/[a-f0-9-]{36}",
            r"arn:aws:cloudfront::[0-9]{12}:function/[A-Z0-9]+",
            r"[A-Z0-9]{13,14}\.cloudfront\.net",

            # EC2 Resources
            r"arn:aws:ec2:[a-z0-9-]+:\d{12}:instance/[a-z0-9]+",
            r"arn:aws:ec2:[a-z0-9-]+:\d{12}:security-group/[a-z0-9]+",
            r"arn:aws:ec2:[a-z0-9-]+:\d{12}:vpc/[a-z0-9]+",
            r"arn:aws:ec2:[a-z0-9-]+:\d{12}:subnet/[a-z0-9]+",
            r"arn:aws:ec2:[a-z0-9-]+:\d{12}:volume/[a-z0-9]+",
            r"arn:aws:ec2:[a-z0-9-]+:\d{12}:snapshot/[a-z0-9]+",
            r"arn:aws:ec2:[a-z0-9-]+:\d{12}:network-interface/[a-z0-9]+",
            r"arn:aws:ec2:[a-z0-9-]+:\d{12}:placement-group/[a-zA-Z0-9-]+",

            # Storage
            r"arn:aws:s3:::[a-z0-9.-]{3,63}",
            r"arn:aws:s3:::[a-z0-9.-]{3,63}/[^*]*",
            r"arn:aws:efs:[a-z0-9-]+:\d{12}:file-system/fs-[a-f0-9]{8,}",
        
            # Database
            r"arn:aws:dynamodb:[a-z0-9-]+:\d{12}:table/[a-zA-Z0-9-_]+",
            r"arn:aws:rds:[a-z0-9-]+:\d{12}:db/[a-zA-Z0-9-]+",
            r"arn:aws:rds:[a-z0-9-]+:\d{12}:cluster/[a-zA-Z0-9-]+",
            r"arn:aws:redshift:[a-z0-9-]+:\d{12}:cluster:[a-zA-Z0-9-]+",

            # Serverless
            r"arn:aws:lambda:[a-z0-9-]+:\d{12}:function:[a-zA-Z0-9-_]+",
            r"arn:aws:lambda:[a-z0-9-]+:\d{12}:function:[a-zA-Z0-9-_]+:[0-9]+",
            r"arn:aws:lambda:[a-z0-9-]+:\d{12}:function:[a-zA-Z0-9-_]+:\$LATEST",
            r"arn:aws:lambda:[a-z0-9-]+:\d{12}:layer:[a-zA-Z0-9-_]+",
            r"arn:aws:lambda:[a-z0-9-]+:\d{12}:layer:[a-zA-Z0-9-_]+:[0-9]+",
            r"arn:aws:lambda:[a-z0-9-]+:\d{12}:event-source-mapping/[a-f0-9-]{36}",
            r"arn:aws:lambda:[a-z0-9-]+:\d{12}:code-signing-config:[a-zA-Z0-9-_]+",
            r"arn:aws:apigateway:[a-z0-9-]+::apis/[a-z0-9]+",

            # ECR
            r"arn:aws:ecr:[a-z0-9-]+:\d{12}:repository/[a-zA-Z0-9_-]+",
            r"arn:aws:ecr:[a-z0-9-]+:\d{12}:repository/[a-zA-Z0-9_-]+/[a-zA-Z0-9_-]+",
            r"\d{12}\.dkr\.ecr\.[a-z0-9-]+\.amazonaws\.com/[a-zA-Z0-9_-]+",
            r"\d{12}\.dkr\.ecr\.[a-z0-9-]+\.amazonaws\.com/[a-zA-Z0-9_-]+:[a-zA-Z0-9_.-]+",
            r"\d{12}\.dkr\.ecr\.[a-z0-9-]+\.amazonaws\.com/[a-zA-Z0-9_-]+@sha256:[a-f0-9]{64}",
            r"arn:aws:ecr-public::[0-9]{12}:repository/[a-zA-Z0-9_-]+",
            r"public\.ecr\.aws/[a-z0-9]+/[a-zA-Z0-9_-]+", 

            # Networking
            r"arn:aws:elasticloadbalancing:[a-z0-9-]+:\d{12}:loadbalancer/[a-zA-Z0-9-]+/[0-9a-f]{8,}",
            r"arn:aws:acm:[a-z0-9-]+:\d{12}:certificate/[0-9a-f-]{36}",
            r"arn:aws:cloudfront::[0-9]{12}:distribution/[A-Z0-9]+",
            r"arn:aws:route53:::hostedzone/[A-Z0-9]+",
            r"arn:aws:wafv2:[a-z0-9-]+:\d{12}:regional/webacl/[a-zA-Z0-9-_]+/[a-f0-9-]+",

            # Security
            r"arn:aws:kms:[a-z0-9-]+:\d{12}:key/[0-9a-f-]{36}",
            r"arn:aws:secretsmanager:[a-z0-9-]+:\d{12}:secret:[A-Za-z0-9/_+=.@-]+",
            r"arn:aws:ssm:[a-z0-9-]+:\d{12}:parameter/[a-zA-Z0-9/_.-]+",
            r"arn:aws:acm:[a-z0-9-]+:\d{12}:certificate/[0-9a-f-]{36}",

            # Monitoring
            r"arn:aws:cloudwatch:[a-z0-9-]+:\d{12}:alarm:[a-zA-Z0-9-_]+",
            r"arn:aws:logs:[a-z0-9-]+:\d{12}:log-group:/[a-zA-Z0-9/_.-]+",

            # Containers
            r"arn:aws:ecs:[a-z0-9-]+:\d{12}:cluster/[a-zA-Z0-9_-]+",
            r"arn:aws:ecs:[a-z0-9-]+:\d{12}:task-definition/[a-zA-Z0-9_-]+:[0-9]+",
            r"arn:aws:ecr:[a-z0-9-]+:\d{12}:repository/[a-zA-Z0-9_-]+",

            # Messaging
            r"arn:aws:sns:[a-z0-9-]+:\d{12}:[a-zA-Z0-9-_]+",
            r"arn:aws:sqs:[a-z0-9-]+:\d{12}:[a-zA-Z0-9-_]+",
            r"arn:aws:events:[a-z0-9-]+:\d{12}:rule/[a-zA-Z0-9-_]+",
        ]
