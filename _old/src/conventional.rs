//! Conventional commit parsing and formatting

use std::{io::BufRead, string::ToString};

use regex::Regex;

use crate::{
    error::{Error, Result},
    utils::StringExt,
};
