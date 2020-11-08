// Copyright (c) 2019-2020 FIUBioRG
// SPDX-License-Identifier: MIT

//! Routes generated by yew-router

use yew_router::prelude::*;

/// App routes
#[derive(Switch, Clone, PartialEq)]
pub enum AppRoute {
    #[to = "/auth{*:sub_route}"]
    Auth(AuthRoute),
    #[to = "/pluma{*:sub_route}"]
    Pluma(PlumaRoute),
    #[to = "/!"]
    Index,
}

// Sub-routes for authentication
#[derive(Switch, Clone, PartialEq)]
pub enum AuthRoute {
    #[to = "/login"]
    Login,
    #[to = "/register"]
    Register,
}

/// PluMA routes
#[derive(Switch, Clone, PartialEq)]
pub enum PlumaRoute {
    #[to = "/plugins"]
    Plugins,
    #[to = "/pipelines"]
    Pipelines,
    #[to = "/!"]
    Index,
}