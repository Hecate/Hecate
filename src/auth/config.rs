use crate::err::HecateError;
use super::Auth;
pub use crate::user::token::Scope as RW;

pub fn not_authed() -> HecateError {
    HecateError::new(401, String::from("You must be logged in to access this resource"), None)
}

#[derive(Clone)]
pub struct AuthContainer(pub CustomAuth);

///
/// Allows a category to be null, public, admin, or user
///
/// This category makes up the majority of endpoints in hecate and is the most
/// flexible
///
fn is_all(scope_type: &str, scope: &Option<String>) -> Result<bool, String> {
    match scope {
        &None => Ok(true),
        &Some(ref scope_str) => {
            match scope_str as &str {
                "public" => Ok(true),
                "admin" => Ok(true),
                "user" => Ok(true),
                _ => Err(format!("Auth Config Error: '{}' must be one of 'public', 'admin', 'user', or null", scope_type)),
            }
        }
    }
}

///
/// Allows a category to be null, self, or admin
///
/// This category is used for CRUD operations against data for a specfic user,
/// not only must the user be logged in but the user can only update their own
/// data
///
fn is_self(scope_type: &str, scope: &Option<String>) -> Result<bool, String> {
    match scope {
        &None => Ok(true),
        &Some(ref scope_str) => {
            match scope_str as &str {
                "self" => Ok(true),
                "admin" => Ok(true),
                _ => Err(format!("Auth Config Error: '{}' must be one of 'self', 'admin', or null", scope_type)),
            }
        }
    }
}

///
/// Allows a category to be null, user, or admin
///
/// This category is used primarily for feature operations. The user must be
/// logged in but can make changes to any feature, including features created
/// by another user
///
fn is_auth(scope_type: &str, scope: &Option<String>) -> Result<bool, String> {
    match scope {
        &None => Ok(true),
        &Some(ref scope_str) => {
            match scope_str as &str {
                "user" => Ok(true),
                "admin" => Ok(true),
                _ => Err(format!("Auth Config Error: '{}' must be one of 'user', 'admin', or null", scope_type)),
            }
        }
    }
}

pub trait ValidAuth {
    fn is_valid(&self) -> Result<bool, String>;
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AuthWebhooks {
    pub get: String,
    pub set: String
}

impl AuthWebhooks {
    fn new() -> Self {
        AuthWebhooks {
            get: String::from("admin"),
            set: String::from("admin")
        }
    }
}

impl ValidAuth for AuthWebhooks {
    fn is_valid(&self) -> Result<bool, String> {
        is_auth("webhooks::get", &Some(self.get.clone()))?;
        is_auth("webhooks::set", &Some(self.set.clone()))?;

        Ok(true)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AuthMeta {
    pub get: String,
    pub set: String
}

impl AuthMeta {
    fn new() -> Self {
        AuthMeta {
            get: String::from("public"),
            set: String::from("admin")
        }
    }
}

impl ValidAuth for AuthMeta {
    fn is_valid(&self) -> Result<bool, String> {
        is_all("meta::get", &Some(self.get.clone()))?;
        is_auth("meta::set", &Some(self.set.clone()))?;

        Ok(true)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AuthClone {
    pub get: Option<String>,
    pub query: Option<String>
}

impl AuthClone {
    fn new() -> Self {
        AuthClone {
            get: Some(String::from("user")),
            query: Some(String::from("user"))
        }
    }
}

impl ValidAuth for AuthClone {
    fn is_valid(&self) -> Result<bool, String> {
        is_all("clone::get", &self.get)?;
        is_all("clone::query", &self.query)?;

        Ok(true)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AuthSchema {
    pub get: Option<String>
}

impl AuthSchema {
    fn new() -> Self {
        AuthSchema {
            get: Some(String::from("public"))
        }
    }
}

impl ValidAuth for AuthSchema {
    fn is_valid(&self) -> Result<bool, String> {
        is_all("schema::get", &self.get)?;

        Ok(true)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AuthStats {
    pub get: Option<String>,
    pub bounds: Option<String>
}

impl AuthStats {
    fn new() -> Self {
        AuthStats {
            get: Some(String::from("public")),
            bounds: Some(String::from("public"))
        }
    }
}

impl ValidAuth for AuthStats {
    fn is_valid(&self) -> Result<bool, String> {
        is_all("stats::get", &self.get)?;
        is_all("stats::bounds", &self.bounds)?;

        Ok(true)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AuthAuth {
    pub get: Option<String>
}

impl AuthAuth {
    fn new() -> Self {
        AuthAuth {
            get: Some(String::from("public"))
        }
    }
}

impl ValidAuth for AuthAuth {
    fn is_valid(&self) -> Result<bool, String> {
        is_all("auth::get", &self.get)?;

        Ok(true)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AuthMVT {
    pub get: Option<String>,
    pub delete: Option<String>,
    pub regen: Option<String>,
    pub meta: Option<String>
}

impl AuthMVT {
    fn new() -> Self {
        AuthMVT {
            get: Some(String::from("public")),
            delete: Some(String::from("admin")),
            regen: Some(String::from("user")),
            meta: Some(String::from("public"))
        }
    }
}

impl ValidAuth for AuthMVT {
    fn is_valid(&self) -> Result<bool, String> {
        is_all("mvt::get", &self.get)?;
        is_all("mvt::regen", &self.regen)?;
        is_all("mvt::delete", &self.regen)?;
        is_all("mvt::meta", &self.meta)?;

        Ok(true)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AuthUser {
    pub info: Option<String>,
    pub list: Option<String>,
    pub create: Option<String>,
    pub create_session: Option<String>
}

impl AuthUser {
    fn new() -> Self {
        AuthUser {
            info: Some(String::from("self")),
            list: Some(String::from("user")),
            create: Some(String::from("public")),
            create_session: Some(String::from("self"))
        }
    }
}

impl ValidAuth for AuthUser {
    fn is_valid(&self) -> Result<bool, String> {
        is_all("user::create", &self.create)?;
        is_all("user::list", &self.list)?;

        is_self("user::create_session", &self.create_session)?;
        is_self("user::info", &self.info)?;

        Ok(true)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AuthStyle {
    pub create: Option<String>,
    pub patch: Option<String>,
    pub set_public: Option<String>,
    pub set_private: Option<String>,
    pub delete: Option<String>,
    pub get: Option<String>,
    pub list: Option<String>
}

impl AuthStyle {
    fn new() -> Self {
        AuthStyle {
            create: Some(String::from("self")),
            patch: Some(String::from("self")),
            set_public: Some(String::from("self")),
            set_private: Some(String::from("self")),
            delete: Some(String::from("self")),
            get: Some(String::from("public")),
            list: Some(String::from("public"))
        }
    }
}

impl ValidAuth for AuthStyle {
    fn is_valid(&self) -> Result<bool, String> {
        is_self("style::create", &self.create)?;
        is_self("style::patch", &self.patch)?;
        is_self("style::set_public", &self.set_public)?;
        is_self("style::set_private", &self.set_private)?;
        is_self("style::delete", &self.delete)?;
        is_all("style::get", &self.get)?;
        is_all("style::list", &self.list)?;

        Ok(true)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AuthDelta {
    pub get: Option<String>,
    pub list: Option<String>,
}

impl AuthDelta {
    fn new() -> Self {
        AuthDelta {
            get: Some(String::from("public")),
            list: Some(String::from("public"))
        }
    }
}

impl ValidAuth for AuthDelta {
    fn is_valid(&self) -> Result<bool, String> {
        is_all("delta::get", &self.get)?;
        is_all("delta::list", &self.list)?;

        Ok(true)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AuthFeature {
    pub force: Option<String>,
    pub create: Option<String>,
    pub get: Option<String>,
    pub history: Option<String>
}

impl AuthFeature {
    fn new() -> Self {
        AuthFeature {
            force: Some(String::from("none")),
            create: Some(String::from("user")),
            get: Some(String::from("public")),
            history: Some(String::from("public"))
        }
    }
}

impl ValidAuth for AuthFeature {
    fn is_valid(&self) -> Result<bool, String> {
        is_auth("feature::create", &self.create)?;
        is_auth("feature::force", &self.force)?;
        is_all("feature::get", &self.get)?;
        is_all("feature::history", &self.history)?;

        Ok(true)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AuthBounds {
    pub list: Option<String>,
    pub create: Option<String>,
    pub delete: Option<String>,
    pub get: Option<String>
}

impl AuthBounds {
    fn new() -> Self {
        AuthBounds {
            list: Some(String::from("public")),
            create: Some(String::from("admin")),
            delete: Some(String::from("admin")),
            get: Some(String::from("public"))
        }
    }
}

impl ValidAuth for AuthBounds {
    fn is_valid(&self) -> Result<bool, String> {
        is_all("bounds::list", &self.list)?;
        is_all("bounds::create", &self.create)?;
        is_all("bounds::delete", &self.create)?;
        is_all("bounds::get", &self.get)?;

        Ok(true)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AuthOSM {
    pub get: Option<String>,
    pub create: Option<String>
}

impl AuthOSM {
    fn new() -> Self {
        AuthOSM {
            get: Some(String::from("public")),
            create: Some(String::from("user"))
        }
    }
}

impl ValidAuth for AuthOSM {
    fn is_valid(&self) -> Result<bool, String> {
        is_all("osm::get", &self.get)?;
        is_auth("osm::create", &self.create)?;

        Ok(true)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CustomAuth {
    pub default: Option<String>,
    pub server: String,
    pub meta: AuthMeta,
    pub webhooks: AuthWebhooks,
    pub stats: Option<AuthStats>,
    pub mvt: Option<AuthMVT>,
    pub schema: Option<AuthSchema>,
    pub auth: Option<AuthAuth>,
    pub user: Option<AuthUser>,
    pub feature: Option<AuthFeature>,
    pub style: Option<AuthStyle>,
    pub delta: Option<AuthDelta>,
    pub bounds: Option<AuthBounds>,
    pub clone: Option<AuthClone>,
    pub osm: Option<AuthOSM>
}

impl ValidAuth for CustomAuth {
    fn is_valid(&self) -> Result<bool, String> {
        is_all("server", &Some(self.server.clone()))?;

        &self.meta.is_valid()?;

        match &self.mvt {
            None => (),
            Some(ref mvt) => { mvt.is_valid()?; }
        };

        match &self.schema {
            None => (),
            Some(ref schema) => { schema.is_valid()?; }
        };

        match &self.user {
            None => (),
            Some(ref user) => { user.is_valid()?; }
        };

        match &self.feature {
            None => (),
            Some(ref feature) => { feature.is_valid()?; }
        };

        match &self.style {
            None => (),
            Some(ref style) => { style.is_valid()?; }
        };

        match &self.delta {
            None => (),
            Some(ref delta) => { delta.is_valid()?; }
        };

        match &self.bounds {
            None => (),
            Some(ref bounds) => { bounds.is_valid()?; }
        };

        match &self.clone {
            None => (),
            Some(ref clone) => { clone.is_valid()?; }
        };

        match &self.osm {
            None => (),
            Some(ref osm) => { osm.is_valid()?; }
        };

        Ok(true)
    }
}

pub fn rw_met(rw: RW, auth: &Auth) -> Result<(), HecateError> {
    if rw == RW::Full && auth.scope == RW::Read {
        return Err(not_authed());
    }

    return Ok(());
}

///
/// Determines whether the current auth state meets or exceeds the
/// requirements of an endpoint
///
fn auth_met(required: &Option<String>, auth: &mut Auth) -> Result<bool, HecateError> {
    match required {
        None => Err(not_authed()),
        Some(req) => match req.as_ref() {
            "public" => Ok(true),
            "admin" => {
                if auth.uid.is_none() || auth.access.is_none() {
                    return Err(not_authed());
                } else if auth.access == Some(String::from("admin")) {
                    return Ok(true);
                } else {
                    return Err(not_authed());
                }
            },
            "user" => {
                if auth.uid.is_some() {
                    return Ok(true);
                } else {
                    return Err(not_authed());
                }
            },
            "self" => {
                //Note: This ensures the user is validated,
                //it is up to the parent caller to ensure
                //the UID of 'self' matches the requested resource

                if auth.uid.is_some() {
                    return Ok(true);
                } else {
                    return Err(not_authed());
                }
            },
            _ => Err(not_authed())
        }
    }
}

impl CustomAuth {
    pub fn new() -> Self {
        CustomAuth {
            default: Some(String::from("public")),
            server: String::from("public"),
            webhooks: AuthWebhooks::new(),
            meta: AuthMeta::new(),
            stats: Some(AuthStats::new()),
            schema: Some(AuthSchema::new()),
            auth: Some(AuthAuth::new()),
            mvt: Some(AuthMVT::new()),
            user: Some(AuthUser::new()),
            feature: Some(AuthFeature::new()),
            style: Some(AuthStyle::new()),
            delta: Some(AuthDelta::new()),
            bounds: Some(AuthBounds::new()),
            clone: Some(AuthClone::new()),
            osm: Some(AuthOSM::new())
        }
    }

    pub fn to_json(&self) -> serde_json::value::Value {
        let json_auth = serde_json::from_str(serde_json::to_string(&self).unwrap().as_str()).unwrap();

        json_auth
    }


    pub fn is_admin(&self, auth: &mut Auth) -> Result<bool, HecateError> {
        auth_met(&Some(String::from("admin")), auth)
    }

    pub fn allows_stats_get(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.stats {
            None => Err(not_authed()),
            Some(stats) => auth_met(&stats.get, auth)
        }
    }

    pub fn allows_stats_bounds(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.stats {
            None => Err(not_authed()),
            Some(stats) => auth_met(&stats.bounds, auth)
        }
    }

    pub fn allows_mvt_get(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.mvt {
            None => Err(not_authed()),
            Some(mvt) => auth_met(&mvt.get, auth)
        }
    }

    pub fn allows_mvt_delete(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.mvt {
            None => Err(not_authed()),
            Some(mvt) => auth_met(&mvt.delete, auth)
        }
    }

    pub fn allows_mvt_regen(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.mvt {
            None => Err(not_authed()),
            Some(mvt) => auth_met(&mvt.regen, auth)
        }
    }

    pub fn allows_mvt_meta(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.mvt {
            None => Err(not_authed()),
            Some(mvt) => auth_met(&mvt.meta, auth)
        }
    }

    pub fn allows_user_list(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.user {
            None => Err(not_authed()),
            Some(user) => auth_met(&user.list, auth)
        }
    }

    pub fn allows_user_create(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.user {
            None => Err(not_authed()),
            Some(user) => auth_met(&user.create, auth)
        }
    }

    pub fn allows_user_info(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.user {
            None => Err(not_authed()),
            Some(user) => auth_met(&user.info, auth)
        }
    }

    pub fn allows_user_create_session(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.user {
            None => Err(not_authed()),
            Some(user) => auth_met(&user.create_session, auth)
        }
    }

    pub fn allows_style_create(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.style {
            None => Err(not_authed()),
            Some(style) => auth_met(&style.create, auth)
        }
    }

    pub fn allows_style_patch(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.style {
            None => Err(not_authed()),
            Some(style) => auth_met(&style.patch, auth)
        }
    }

    pub fn allows_style_set_public(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.style {
            None => Err(not_authed()),
            Some(style) => auth_met(&style.set_public, auth)
        }
    }

    pub fn allows_style_set_private(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.style {
            None => Err(not_authed()),
            Some(style) => auth_met(&style.set_private, auth)
        }
    }

    pub fn allows_style_delete(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.style {
            None => Err(not_authed()),
            Some(style) => auth_met(&style.delete, auth)
        }
    }

    pub fn allows_style_get(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.style {
            None => Err(not_authed()),
            Some(style) => auth_met(&style.get, auth)
        }
    }

    pub fn allows_style_list(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.style {
            None => Err(not_authed()),
            Some(style) => auth_met(&style.list, auth)
        }
    }

    pub fn allows_delta_get(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.delta {
            None => Err(not_authed()),
            Some(delta) => auth_met(&delta.get, auth)
        }
    }

    pub fn allows_delta_list(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.delta {
            None => Err(not_authed()),
            Some(delta) => auth_met(&delta.list, auth)
        }
    }

    pub fn allows_clone_get(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.clone {
            None => Err(not_authed()),
            Some(clone) => auth_met(&clone.get, auth)
        }
    }

    pub fn allows_clone_query(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.clone {
            None => Err(not_authed()),
            Some(clone) => auth_met(&clone.query, auth)
        }
    }

    pub fn allows_bounds_get(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.bounds {
            None => Err(not_authed()),
            Some(bounds) => auth_met(&bounds.get, auth)
        }
    }

    pub fn allows_bounds_create(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.bounds {
            None => Err(not_authed()),
            Some(bounds) => auth_met(&bounds.create, auth)
        }
    }

    pub fn allows_bounds_delete(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.bounds {
            None => Err(not_authed()),
            Some(bounds) => auth_met(&bounds.delete, auth)
        }
    }

    pub fn allows_bounds_list(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.bounds {
            None => Err(not_authed()),
            Some(bounds) => auth_met(&bounds.list, auth)
        }
    }

    pub fn allows_feature_create(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.feature {
            None => Err(not_authed()),
            Some(feature) => auth_met(&feature.create, auth)
        }
    }

    pub fn allows_feature_force(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.feature {
            None => Err(not_authed()),
            Some(feature) => auth_met(&feature.force, auth)
        }
    }

    pub fn allows_feature_get(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.feature {
            None => Err(not_authed()),
            Some(feature) => auth_met(&feature.get, auth)
        }
    }

    pub fn allows_feature_history(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.feature {
            None => Err(not_authed()),
            Some(feature) => auth_met(&feature.history, auth)
        }
    }

    pub fn allows_schema_get(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.schema {
            None => Err(not_authed()),
            Some(schema) => auth_met(&schema.get, auth)
        }
    }

    pub fn allows_auth_get(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.auth {
            None => Err(not_authed()),
            Some(a) => auth_met(&a.get, auth)
        }
    }

    pub fn allows_osm_get(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.osm {
            None => Err(not_authed()),
            Some(osm) => auth_met(&osm.get, auth)
        }
    }

    pub fn allows_osm_create(&self, auth: &mut Auth, rw: RW) -> Result<bool, HecateError> {
        rw_met(rw, &auth)?;

        match &self.osm {
            None => Err(not_authed()),
            Some(osm) => auth_met(&osm.create, auth)
        }
    }
}
