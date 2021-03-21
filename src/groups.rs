use crate::commands::icon::*;
use crate::commands::meta::*;

use serenity::framework::standard::macros::group;

#[group]
#[commands(ping, shutdown)]
pub struct General;

#[group]
#[commands(icon)]
pub struct Icon;
