use rand::distributions::Alphanumeric;
use rand::prelude::*;
use reqwest::RequestBuilder;

fn generate_random_string(length: u8) -> String {
    let rng = rand::thread_rng();
    let random_bytes = rng
        .sample_iter(&Alphanumeric)
        .take(length.into())
        .collect::<Vec<u8>>();

    String::from_utf8(random_bytes).unwrap()
}

/// A struct that represents the data needed to make sure that the replit api thinks that we are a real user.
pub struct ReplitApiHumanData {
    // Cookies
    pub gating_id: String,
    pub gfa_ref: String,
    pub gfa_landed_on: String,
    pub gfa_campaign: String,
    pub ajs_user_id: String,
    pub ajs_anonymous_id: String,
    pub ld_uid: String,
    pub amplitude_session_id: String,
    pub _dd_s: String,
    // Headers
    pub user_agent: String,
    pub user_agent_platform: String,
    pub x_client_version: String,
    pub x_requested_with: String,
}

impl Default for ReplitApiHumanData {
    /// Randomly generates the data needed to make sure that the replit api thinks that we are a real user.
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let gating_id = format!(
            "{}-{}-{}-{}-{}",
            generate_random_string(8),
            generate_random_string(4),
            generate_random_string(4),
            generate_random_string(4),
            generate_random_string(12)
        );
        let gfa_ref = "https://www.google.com/".to_string();
        // Why not?
        let gfa_landed_on = "/talk/ask/How-to-install-Docker/39255".to_string();
        let gfa_campaign = "hackwithus-console".to_string();
        let ajs_user_id = rng.gen_range(0..10000000).to_string();
        let ajs_anonymous_id = format!(
            "{}-{}-{}-{}-{}",
            generate_random_string(8),
            generate_random_string(4),
            generate_random_string(4),
            generate_random_string(4),
            generate_random_string(12)
        );
        let ld_uid = rng.gen_range(0..10000000).to_string();
        let amplitude_session_id = rng.gen_range(0..10000000).to_string();
        let _dd_s = format!(
            "logs=1&id={}-{}-{}-{}-{}&created={}&expire={}",
            generate_random_string(8),
            generate_random_string(4),
            generate_random_string(4),
            generate_random_string(4),
            generate_random_string(12),
            rng.gen_range(1000000..10000000),
            rng.gen_range(1000000..10000000)
        );
        let user_agent = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.114 Safari/537.36".to_string();
        let user_agent_platform = "Linux x86_64".to_string();
        let x_client_version = "f0c2eb6".to_string();
        let x_requested_with = "XMLHttpRequest".to_string();

        Self {
            gating_id,
            gfa_ref,
            gfa_landed_on,
            gfa_campaign,
            ajs_user_id,
            ajs_anonymous_id,
            ld_uid,
            amplitude_session_id,
            _dd_s,
            user_agent,
            user_agent_platform,
            x_client_version,
            x_requested_with,
        }
    }
}

impl ReplitApiHumanData {
    /// Pads the request with the data needed to make sure that the replit api thinks that we are a real user.
    pub fn pad_request(self, request: RequestBuilder) -> RequestBuilder {
        request
            .header("accept", "*/*")
            .header("accept-language", "en-US,en;q=0.9")
            .header("content-type", "application/json")
            .header("sec-ch-ua-mobile", "?0")
            .header("sec-fetch-dest", "empty")
            .header("sec-fetch-mode", "cors")
            .header("sec-fetch-site", "same-origin")
            .header("sec-gpc", "1")
            .header("sec-ch-ua", &self.user_agent)
            .header("sec-ch-ua-platform", &self.user_agent_platform)
            .header("x-client-version", self.x_client_version)
            .header("x-requested-with", self.x_requested_with)
            // Pads the cookie
            .header("cookie", format!("gating_id={}; gfa_ref={}; gfa_landed_on={}; gfa_campaign={}; ajs_user_id={}; ajs_anonymous_id={}; ld_uid={}; amplitude_session_id={}; _dd_s={}", self.gating_id, self.gfa_ref, self.gfa_landed_on, self.gfa_campaign, self.ajs_user_id, self.ajs_anonymous_id, self.ld_uid, self.amplitude_session_id, self._dd_s))
            .header("user-agent", &self.user_agent)
            .header("user-agent-platform", &self.user_agent_platform)
    }
}

/// A replit API client
#[derive(Default)]
pub struct ReplitApiClient {
    client: reqwest::Client,
    human_data: ReplitApiHumanData,
}

impl ReplitApiClient {
    /// Creates a new repl
    pub async fn create_repl(self, repl_name: &str) {
        /*
        An example of how to create a repl using the replit api (using JavaScript fetch):
        fetch("https://replit.com/graphql", {
          "headers": {
            "accept": "*\/\*",
            "accept-language": "en-US,en;q=0.9",
            "content-type": "application/json",
            "sec-ch-ua": "\"Brave\";v=\"111\", \"Not(A:Brand\";v=\"8\", \"Chromium\";v=\"111\"",
            "sec-ch-ua-mobile": "?0",
            "sec-ch-ua-platform": "\"macOS\"",
            "sec-fetch-dest": "empty",
            "sec-fetch-mode": "cors",
            "sec-fetch-site": "same-origin",
            "sec-gpc": "1",
            "x-client-version": "f0c2eb6",
            "x-requested-with": "XMLHttpRequest",
            "cookie": "gating_id=53e12b5c-b550-40f8-854c-a3f52fb49646; gfa_ref=https://www.google.com/; gfa_landed_on=/talk/ask/How-to-install-Docker/39255; gfa_campaign=hackwithus-console; ajs_user_id=3740205; ajs_anonymous_id=8a09cd6a-8981-46ca-be38-5d3ceac2e70a; ld_uid=3740205; replit_authed=1; amplitudeSessionId=1679346043; _dd_s=logs=1&id=8fbd2b44-3661-4d3b-a7f8-ca408f54b191&created=1679346044023&expire=1679346969437",
            "Referer": "https://replit.com/new",
            "Referrer-Policy": "strict-origin-when-cross-origin"
          },
          "body": "[{\"operationName\":\"CreateReplFormCreateRepl\",\"variables\":{\"input\":{\"title\":\"test1\",\"folderId\":null,\"isPrivate\":false,\"originId\":\"a97a6fe7-adec-4350-8337-65846053a082\",\"replReleaseId\":\"23046e71-d0f6-4a99-afda-f318424d2e70\"},\"isTitleAutoGenerated\":false},\"query\":\"mutation CreateReplFormCreateRepl($input: CreateReplInput!, $isTitleAutoGenerated: Boolean!) {\\n  createRepl(input: $input, isTitleAutoGenerated: $isTitleAutoGenerated) {\\n    ... on Repl {\\n      ...CreateReplFormRepl\\n      __typename\\n    }\\n    ... on UserError {\\n      message\\n      __typename\\n    }\\n    __typename\\n  }\\n}\\n\\nfragment CreateReplFormRepl on Repl {\\n  id\\n  ...TemplateSelector2Repl\\n  __typename\\n}\\n\\nfragment TemplateSelector2Repl on Repl {\\n  id\\n  url\\n  title\\n  iconUrl\\n  templateLabel\\n  nixedLanguage\\n  isPrivate\\n  isRenamed\\n  language\\n  likeCount\\n  description(plainText: true)\\n  deployment {\\n    id\\n    activeRelease {\\n      id\\n      __typename\\n    }\\n    __typename\\n  }\\n  owner {\\n    ... on User {\\n      id\\n      username\\n      __typename\\n    }\\n    ... on Team {\\n      id\\n      username\\n      __typename\\n    }\\n    __typename\\n  }\\n  ...TemplateReplCardRepl\\n  __typename\\n}\\n\\nfragment TemplateReplCardRepl on Repl {\\n  id\\n  iconUrl\\n  templateCategory\\n  title\\n  description(plainText: true)\\n  releasesForkCount\\n  templateLabel\\n  likeCount\\n  url\\n  owner {\\n    ... on User {\\n      id\\n      ...TemplateReplCardFooterUser\\n      __typename\\n    }\\n    ... on Team {\\n      id\\n      ...TemplateReplCardFooterTeam\\n      __typename\\n    }\\n    __typename\\n  }\\n  deployment {\\n    id\\n    activeRelease {\\n      id\\n      __typename\\n    }\\n    __typename\\n  }\\n  publishedAs\\n  __typename\\n}\\n\\nfragment TemplateReplCardFooterUser on User {\\n  id\\n  username\\n  image\\n  url\\n  __typename\\n}\\n\\nfragment TemplateReplCardFooterTeam on Team {\\n  id\\n  username\\n  image\\n  url\\n  __typename\\n}\\n\"}]",
          "method": "POST"
        });
        */

        let mut request = self.client.post("https://replit.com/graphql");
        request = self.human_data.pad_request(request);
        request = request.body(format!(
            r#"{{
                "operationName": "CreateReplFormCreateRepl",
                "variables": {{
                    "input": {{
                        "title": "{repl_name}",
                        "folderId": null,
                        "isPrivate": false,
                        "originId": "a97a6fe7-adec-4350-8337-65846053a082",
                        "replReleaseId": "23046e71-d0f6-4a99-afda-f318424d2e70"
                    }},
                    "isTitleAutoGenerated": false
                }},
            }}"#
        ));
        request.send().await.unwrap();
    }
}
