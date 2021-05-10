use crate::commands::banner::*;
use crate::commands::icon::*;
use crate::commands::meta::*;
use crate::commands::admin::*;

use serenity::framework::standard::macros::group;

#[group]
#[commands(ping, shutdown)]
pub struct General;

#[group]
#[commands(icon, banner)]
pub struct Storage;


#[group]
#[commands(list_roles, allow_roles, clear_roles)]
pub struct Admin;