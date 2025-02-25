use reqwest::Client;
use crate::utils::build_query_string;
use crate::models::WordElement;

#[derive(Debug, Clone)]
pub struct WordsEndpoint {
    client: Client,
    ml: Option<String>,
    sl: Option<String>,
    sp: Option<String>,
    rel_syn: Option<String>,
    rel_trg: Option<String>,
    rel_ant: Option<String>,
    rel_jja: Option<String>,
    rel_jjb: Option<String>,
    rel_spc: Option<String>,
    rel_gen: Option<String>,
    rel_com: Option<String>,
    rel_par: Option<String>,
    rel_bga: Option<String>,
    rel_bgb: Option<String>,
    rel_hom: Option<String>,
    rel_cns: Option<String>,
    lc: Option<String>,
    rc: Option<String>,
    md: Option<String>,
    qe: Option<String>,
    max: Option<u32>,
}

impl WordsEndpoint {
    pub fn new(client: Client) -> Self {
        WordsEndpoint {
            client,
            ml: None,
            sl: None,
            sp: None,
            rel_syn: None,
            rel_trg: None,
            rel_ant: None,
            rel_jja: None,
            rel_jjb: None,
            rel_spc: None,
            rel_gen: None,
            rel_com: None,
            rel_par: None,
            rel_bga: None,
            rel_bgb: None,
            rel_hom: None,
            rel_cns: None,
            lc: None,
            rc: None,
            md: None,
            qe: None,
            max: None,
        }
    }

    pub fn means_like(&self, term: &str) -> Self {
        let mut new_endpoint = self.clone();
        new_endpoint.ml = Some(term.to_string());
        new_endpoint
    }

    pub fn sounds_like(&self, term: &str) -> Self {
        let mut new_endpoint = self.clone();
        new_endpoint.sl = Some(term.to_string());
        new_endpoint
    }

    pub fn spelled_like(&self, term: &str) -> Self {
        let mut new_endpoint = self.clone();
        new_endpoint.sp = Some(term.to_string());
        new_endpoint
    }

    pub fn related_synonyms(&self, term: &str) -> Self {
        let mut new_endpoint = self.clone();
        new_endpoint.rel_syn = Some(term.to_string());
        new_endpoint
    }

    pub fn related_triggers(&self, term: &str) -> Self {
        let mut new_endpoint = self.clone();
        new_endpoint.rel_trg = Some(term.to_string());
        new_endpoint
    }

    pub fn related_antonyms(&self, term: &str) -> Self {
        let mut new_endpoint = self.clone();
        new_endpoint.rel_ant = Some(term.to_string());
        new_endpoint
    }

    pub fn related_jja(&self, term: &str) -> Self {
        let mut new_endpoint = self.clone();
        new_endpoint.rel_jja = Some(term.to_string());
        new_endpoint
    }

    pub fn related_jjb(&self, term: &str) -> Self {
        let mut new_endpoint = self.clone();
        new_endpoint.rel_jjb = Some(term.to_string());
        new_endpoint
    }

    pub fn related_spc(&self, term: &str) -> Self {
        let mut new_endpoint = self.clone();
        new_endpoint.rel_spc = Some(term.to_string());
        new_endpoint
    }

    pub fn related_gen(&self, term: &str) -> Self {
        let mut new_endpoint = self.clone();
        new_endpoint.rel_gen = Some(term.to_string());
        new_endpoint
    }

    pub fn related_com(&self, term: &str) -> Self {
        let mut new_endpoint = self.clone();
        new_endpoint.rel_com = Some(term.to_string());
        new_endpoint
    }

    pub fn related_par(&self, term: &str) -> Self {
        let mut new_endpoint = self.clone();
        new_endpoint.rel_par = Some(term.to_string());
        new_endpoint
    }

    pub fn related_bga(&self, term: &str) -> Self {
        let mut new_endpoint = self.clone();
        new_endpoint.rel_bga = Some(term.to_string());
        new_endpoint
    }

    pub fn related_bgb(&self, term: &str) -> Self {
        let mut new_endpoint = self.clone();
        new_endpoint.rel_bgb = Some(term.to_string());
        new_endpoint
    }

    pub fn related_hom(&self, term: &str) -> Self {
        let mut new_endpoint = self.clone();
        new_endpoint.rel_hom = Some(term.to_string());
        new_endpoint
    }

    pub fn related_cns(&self, term: &str) -> Self {
        let mut new_endpoint = self.clone();
        new_endpoint.rel_cns = Some(term.to_string());
        new_endpoint
    }

    pub fn left_context(&self, term: &str) -> Self {
        let mut new_endpoint = self.clone();
        new_endpoint.lc = Some(term.to_string());
        new_endpoint
    }

    pub fn right_context(&self, term: &str) -> Self {
        let mut new_endpoint = self.clone();
        new_endpoint.rc = Some(term.to_string());
        new_endpoint
    }

    pub fn meta_data(&self, term: &str) -> Self {
        let mut new_endpoint = self.clone();
        new_endpoint.md = Some(term.to_string());
        new_endpoint
    }

    pub fn query_echo(&self, term: &str) -> Self {
        let mut new_endpoint = self.clone();
        new_endpoint.qe = Some(term.to_string());
        new_endpoint
    }

    pub fn max(&self, max: u32) -> Self {
        let mut new_endpoint = self.clone();
        new_endpoint.max = Some(max);
        new_endpoint
    }

    pub async fn call(&self) -> Result<Vec<WordElement>, Box<dyn std::error::Error>> {
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(ml) = &self.ml {
            params.push(("ml", ml.clone()));
        }
        if let Some(sl) = &self.sl {
            params.push(("sl", sl.clone()));
        }
        if let Some(sp) = &self.sp {
            params.push(("sp", sp.clone()));
        }
        if let Some(rel_syn) = &self.rel_syn {
            params.push(("rel_syn", rel_syn.clone()));
        }
        if let Some(rel_trg) = &self.rel_trg {
            params.push(("rel_trg", rel_trg.clone()));
        }
        if let Some(rel_ant) = &self.rel_ant {
            params.push(("rel_ant", rel_ant.clone()));
        }
        if let Some(rel_jja) = &self.rel_jja {
            params.push(("rel_jja", rel_jja.clone()));
        }
        if let Some(rel_jjb) = &self.rel_jjb {
            params.push(("rel_jjb", rel_jjb.clone()));
        }
        if let Some(rel_spc) = &self.rel_spc {
            params.push(("rel_spc", rel_spc.clone()));
        }
        if let Some(rel_gen) = &self.rel_gen {
            params.push(("rel_gen", rel_gen.clone()));
        }
        if let Some(rel_com) = &self.rel_com {
            params.push(("rel_com", rel_com.clone()));
        }
        if let Some(rel_par) = &self.rel_par {
            params.push(("rel_par", rel_par.clone()));
        }
        if let Some(rel_bga) = &self.rel_bga {
            params.push(("rel_bga", rel_bga.clone()));
        }
        if let Some(rel_bgb) = &self.rel_bgb {
            params.push(("rel_bgb", rel_bgb.clone()));
        }
        if let Some(rel_hom) = &self.rel_hom {
            params.push(("rel_hom", rel_hom.clone()));
        }
        if let Some(rel_cns) = &self.rel_cns {
            params.push(("rel_cns", rel_cns.clone()));
        }
        if let Some(lc) = &self.lc {
            params.push(("lc", lc.clone()));
        }
        if let Some(rc) = &self.rc {
            params.push(("rc", rc.clone()));
        }
        if let Some(md) = &self.md {
            params.push(("md", md.clone()));
        }
        if let Some(qe) = &self.qe {
            params.push(("qe", qe.clone()));
        }
        if let Some(max) = &self.max {
            params.push(("max", max.to_string()));
        }

        let query_string = build_query_string(&params);
        let url = format!("https://api.datamuse.com/words?{}", query_string);

        let response = self.client.get(&url).send().await?;
        let word_list: Vec<WordElement> = response.json().await?;

        Ok(word_list)
    }
}
