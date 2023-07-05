use crate::{providers::base_provider::BaseProvider, utils::vec_of_strings};

#[derive(Debug, Clone)]
pub struct GithubTheSpeedXProxyListSocks5Provider {
    pub base: BaseProvider,
    pub url: String,
    pub pattern: String,
}

impl GithubTheSpeedXProxyListSocks5Provider {
    pub async fn get_proxies(&mut self) -> Vec<(String, u16, Vec<String>)> {
        let req = self.base.build_get_request(self.url.clone());
        let html = self.base.get_html(req).await;
        let proxies = self.base.find_proxies(self.pattern.clone(), html.as_str());
        self.base.update_stack(&proxies).await;

        proxies
    }
}

impl Default for GithubTheSpeedXProxyListSocks5Provider {
    fn default() -> Self {
        Self {
            base: BaseProvider {
                proto: vec_of_strings!["SOCKS5"],
                domain: "TheSpeedX/SOCKS-List/socks5".to_string(),
                ..Default::default()
            },
            url: "https://raw.githubusercontent.com/TheSpeedX/SOCKS-List/master/socks5.txt"
                .to_string(),
            pattern: r#"(?P<ip>(?:\d+\.?){4})\:(?P<port>\d+)"#.to_string(),
        }
    }
}
