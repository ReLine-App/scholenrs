use std::collections::HashMap;

use scraper::{Html, Selector, ElementRef};
use ureq::{Agent, AgentBuilder, Request, Response};
use url::Url;

use crate::{error::{ScholenError, ScholenResult}, models::student::Student};

pub const URL: &str = "https://www.scholen.net";

pub struct Client {
    username: String,
    token: String,
    group: String,
    client: ureq::Agent
}

impl Client {
    pub fn new(username: String, token: String, group: String) -> Client {
        let agent: Agent = AgentBuilder::new()
            .redirects(0)
            .build();
        
        Client {
            username,
            token,
            group,
            client: agent
        }
    }

    fn apply_authentication(&self, request: Request) -> Request {
        request.query("username", &self.username)
               .query("password", &self.token)
               .query("groups", &self.group)
    }

    fn get_students_html(&self) -> ScholenResult<String> {
        let mut request: Request = self.client.get(&(URL.to_owned() + "/demo/show_import.php"));
        request = self.apply_authentication(request);

        let resp: Response = request.call().map_err(|e| ScholenError::ApplicationUnreachable(e.to_string()))?;

        Ok(resp.into_string().map_err(|_| ScholenError::ParseError)?)
    }

    fn parse_student_from_element(&self, el: ElementRef) -> ScholenResult<Student> {
        let href = match el.value().attr("href") {
            Some(x) => x,
            None => ""
        };

        let url = match Url::parse(&(URL.to_owned() + href)) {
            Ok(url) => url,
            Err(_) => return Err(ScholenError::ParseError)
        };

        let query: HashMap<String, String> =  url.query_pairs().into_owned().collect();

        let id = match query.get("id") {
            Some(id) => id.to_owned(),
            None => return Err(ScholenError::ParseError)
        };

        Ok(Student {
            id,
            name: el.inner_html()
        })
    }

    fn parse_students_html(&self, html: String) -> ScholenResult<Vec<Student>> {
        let doc: Html = Html::parse_document(&html);

        let selector: Selector = Selector::parse("table.list_tutors div.tutor-pic a[style=\"float:left;\"").map_err(|_| ScholenError::ParseError)?;

        let mut students = Vec::new();
        for el in doc.select(&selector) {
            let student = self.parse_student_from_element(el)?;
            students.push(student);
        }

        Ok(students)
    }

    pub fn get_students(&self) -> ScholenResult<Vec<Student>> {
        let html: String = self.get_students_html()?;
        
        let students: Vec<Student> = self.parse_students_html(html)?;        
        
        Ok(students)

    }
}