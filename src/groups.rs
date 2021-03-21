use crate::commands::meta::*;
use crate::commands::icon::*;

use serenity::framework::standard::macros::group;

#[group]
#[commands(ping, shutdown)]
pub struct General;

#[group]
#[commands(icon)]
pub struct Icon;
