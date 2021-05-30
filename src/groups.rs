use crate::commands::admin::*;
use crate::commands::add::*;
use crate::commands::clear::*;
use crate::commands::del::*;
use crate::commands::get::*;
use crate::commands::list::*;
use crate::commands::meta::*;
use crate::commands::set::*;
use crate::commands::shuffle::*;

use serenity::framework::standard::macros::group;

#[group]
#[commands(ping, shutdown)]
pub struct Meta;

#[group]
#[commands(get, set, list, add, del, clear, shuffle)]
pub struct General;

#[group]
#[commands(list_roles, allow_roles, clear_roles)]
pub struct Admin;
