use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use url::Url;
use crate::utils::fetch::{fetch_object};

const URI: &str = "https://raw.githubusercontent.com/mirage2032/resume/refs/heads/master/resume.json";

#[derive(Clone,Serialize,Deserialize)]
pub struct Name {
    pub first: String,
    pub middle: String,
    pub last:String
}

#[derive(Clone,Serialize,Deserialize)]
pub struct Location {
    pub city:String,
    pub country:String
}
#[derive(Clone,Serialize,Deserialize)]
pub struct Website {
    pub name: String,
    pub uri: Url
}
#[derive(Clone,Serialize,Deserialize)]
pub struct Contact {
    pub phone: String,
    pub email: String,
    pub location: Location,
    pub website:Vec<Website>
}

#[derive(Clone,Serialize,Deserialize)]
pub struct Interval{
    pub start:NaiveDate,
    pub end:Option<NaiveDate>
}
#[derive(Clone,Serialize,Deserialize)]
pub struct Education{
    pub name:String,
    pub location:Location,
    pub degree:String,
    pub major:String,
    pub interval:Interval
}

#[derive(Clone,Serialize,Deserialize)]
pub struct Work{
    pub name:String,
    pub position:String,
    pub interval:Interval,
    pub lines:Vec<String>,
    pub skills:Vec<String>
}

#[derive(Clone,Serialize,Deserialize)]
pub struct Language{
    pub name:String,
    pub level:String,
}

#[derive(Clone,Serialize,Deserialize)]
pub struct Technologies{
    pub languages:Vec<String>,
    pub others:Vec<String>
}

#[derive(Clone,Serialize,Deserialize)]
pub struct Project{
    pub name:String,
    pub description:String,
    pub uri:Url
}
#[derive(Clone,Serialize,Deserialize)]
pub struct Resume {
    pub name:Name,
    pub photo:Url,
    pub description:String,
    pub contact:Contact,
    pub education:Vec<Education>,
    pub work:Vec<Work>,
    pub languages:Vec<Language>,
    pub skills:Vec<String>,
    pub technologies:Technologies,
    pub projects:Vec<Project>
}

pub async fn fetch_resume() -> Result<Resume,String>{
    fetch_object(URI).await
}