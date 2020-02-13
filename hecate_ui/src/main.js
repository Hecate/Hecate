import Vue from 'vue';
import App from './App.vue';

window.onload = () => {
    new Vue({
        el: '#app',
        render: h => h(App)
    });
}

/**
 * Format non-200 status codes as errors
 * and call the callback
 *
 * @param {Object} response fetch response
 * @param {function} cb Callback function
 */
function res2err(response, cb) {
    let err = new Error(response.status + ':' + response.statusText);
    err.status = response.status;

    return cb(err);
}

window.hecate = {
    auth: {
        get: function(cb) {
            fetch(`${window.location.protocol}//${window.location.host}/api/auth`, {
                method: 'GET',
                credentials: 'same-origin'
            }).then((response) => {
                if (response.status !== 200) return res2err(response, cb);

                return response.json();
            }).then((response) => {
                return cb(null, response);
            }).catch((err) => {
                return cb(err);
            });
        }
    },
    schema: {
        get: function(cb) {
            fetch(`${window.location.protocol}//${window.location.host}/api/schema`, {
                method: 'GET',
                credentials: 'same-origin'
            }).then((response) => {
                if (response.status === 404) return cb(null, false);
                if (response.status !== 200) return res2err(response, cb);
                return response.json();
            }).then((response) => {
                return cb(null, response);
            }).catch((err) => {
                return cb(err);
            });
        }
    },
    feature: {
        get: function(id, cb) {
            if (!id) return cb(new Error('feature id required'));

            fetch(`${window.location.protocol}//${window.location.host}/api/data/feature/${id}`, {
                method: 'GET',
                credentials: 'same-origin'
            }).then((response) => {
                if (response.status === 404) return cb(null, false);
                if (response.status !== 200) return res2err(response, cb);
                return response.json();
            }).then((response) => {
                return cb(null, response);
            }).catch((err) => {
                return cb(err);
            });
        },
        history: function(id, cb) {
            if (!id) return cb(new Error('feature id required'));

            fetch(`${window.location.protocol}//${window.location.host}/api/data/feature/${id}/history`, {
                method: 'GET',
                credentials: 'same-origin'
            }).then((response) => {
                if (response.status === 404) return cb(null, false);
                if (response.status !== 200) return res2err(response, cb);
                return response.json();
            }).then((response) => {
                return cb(null, response);
            }).catch((err) => {
                return cb(err);
            });
        }
    },
    features: {
        point: function(pt, cb) {
            if (!pt || !Array.isArray(pt) || pt.length !== 2) return cb(new Error('point required'));

            fetch(`${window.location.protocol}//${window.location.host}/api/data/features?point=${encodeURIComponent(pt[0] + ',' + pt[1])}`, {
                method: 'GET',
                credentials: 'same-origin'
            }).then((response) => {
                if (response.status !== 200) return res2err(response, cb);
                return response.text();
            }).then((response) => {
                response = response.split('\n');
                response.pop(); // Remove EOS ctrl char
                response = response.map((b) => {
                    return JSON.parse(b);
                });

                if (response.length === 0) {
                    return cb(null, false);
                } else {
                    return cb(null, response);
                }
            }).catch((err) => {
                return cb(err);
            });

        }
    },
    tiles: {
        clear: function(cb) {
            fetch(`${window.location.protocol}//${window.location.host}/api/tiles`, {
                method: 'DELETE',
                credentials: 'same-origin'
            }).then((response) => {
                if (response.status !== 200) return res2err(response, cb);
                return cb();
            }).catch((err) => {
                return cb(err);
            });
        }
    },
    meta: {
        get: function(key, cb) {
            if (!key) return cb(new Error('key required'));

            fetch(`${window.location.protocol}//${window.location.host}/api/meta/${key}`, {
                method: 'GET',
                credentials: 'same-origin'
            }).then((response) => {
                if (response.status !== 200) return cb(new Error(response.status + ':' + response.statusText));
                return response.json();
            }).then((response) => {
                return cb(null, response);
            }).catch((err) => {
                return cb(err);
            });
        }
    },
    user: {
        info: function(cb) {
            fetch(`${window.location.protocol}//${window.location.host}/api/user/info`, {
                method: 'GET',
                credentials: 'same-origin'
            }).then((response) => {
                if (response.status !== 200) return res2err(response, cb);
                return response.json();
            }).then((users) => {
                return cb(null, users);
            }).catch((err) => {
                return cb(err);
            });
        },
        tokens: function(cb) {
            fetch(`${window.location.protocol}//${window.location.host}/api/user/tokens`, {
                method: 'GET',
                credentials: 'same-origin'
            }).then((response) => {
                if (response.status !== 200) return res2err(response, cb);
                return response.json();
            }).then((tokens) => {
                return cb(null, tokens.map((token) => {
                    token.scope = token.scope.toLowerCase();
                    if (token.expiry) {
                        token.expiry = token.expiry.replace(/\..*$/, '');
                    }
                    return token;
                }));
            }).catch((err) => {
                return cb(err);
            });
        },
        token: {
            create: function(token, cb) {
                fetch(`${window.location.protocol}//${window.location.host}/api/user/token`, {
                    method: 'POST',
                    credentials: 'same-origin',
                    headers: new Headers({
                        'Content-Type': 'application/json'
                    }),
                    body: JSON.stringify(token)
                }).then((response) => {
                    if (response.status !== 200) return res2err(response, cb);
                    return response.json();
                }).then((token) => {
                    return cb(null, token);
                }).catch((err) => {
                    return cb(err);
                });
            },
            delete: function(token, cb) {
                if (!token) return cb(new Error('Token ID required'));

                fetch(`${window.location.protocol}//${window.location.host}/api/user/token/${token}`, {
                    method: 'DELETE',
                    credentials: 'same-origin',
                    headers: new Headers({
                        'Content-Type': 'application/json'
                    }),
                    body: JSON.stringify(token)
                }).then((response) => {
                    if (response.status !== 200) return res2err(response, cb);
                    return response.json();
                }).then((token) => {
                    return cb(null, token);
                }).catch((err) => {
                    return cb(err);
                });
            }
        }
    },
    users: {
        list: function(filter, cb) {
            fetch(`${window.location.protocol}//${window.location.host}/api/users?filter=${filter}`, {
                method: 'GET',
                credentials: 'same-origin'
            }).then((response) => {
                if (response.status !== 200) return res2err(response, cb);
                return response.json();
            }).then((users) => {
                return cb(null, users);
            }).catch((err) => {
                return cb(err);
            });
        },
        get: function(user_id, cb) {
            fetch(`${window.location.protocol}//${window.location.host}/api/user/${user_id}`, {
                method: 'GET',
                credentials: 'same-origin'
            }).then((response) => {
                if (response.status !== 200) return cb(new Error(response.status + ':' + response.statusText));
                return response.json();
            }).then((user) => {
                return cb(null, user);
            }).catch((err) => {
                return cb(err);
            });
        },
        update: function(user, cb) {
            fetch(`${window.location.protocol}//${window.location.host}/api/user/${user.id}`, {
                method: 'POST',
                credentials: 'same-origin',
                headers: new Headers({
                    'Content-Type': 'application/json'
                }),
                body: JSON.stringify(user)
            }).then((response) => {
                if (response.status !== 200) return res2err(response, cb);
                return response.json();
            }).then((user) => {
                return cb(null, user);
            }).catch((err) => {
                return cb(err);
            });
        }
    },
    webhooks: {
        list: function(cb) {
            fetch(`${window.location.protocol}//${window.location.host}/api/webhooks`, {
                method: 'GET',
                credentials: 'same-origin'
            }).then((response) => {
                if (response.status !== 200) return res2err(response, cb);
                return response.json();
            }).then((hooks) => {
                return cb(null, hooks);
            }).catch((err) => {
                return cb(err);
            });
        },
        get: function(id, cb) {
            if (!id) return cb(new Error('Webhook ID required'));

            fetch(`${window.location.protocol}//${window.location.host}/api/webhooks/${id}`, {
                method: 'GET',
                credentials: 'same-origin'
            }).then((response) => {
                if (response.status !== 200) return res2err(response, cb);
                return response.json();
            }).then((hook) => {
                return cb(null, hook);
            }).catch((err) => {
                return cb(err);
            });
        },
        delete: function(id, cb) {
            if (!id) return cb(new Error('Webhook ID required'));

            fetch(`${window.location.protocol}//${window.location.host}/api/webhooks/${id}`, {
                method: 'DELETE',
                credentials: 'same-origin'
            }).then((response) => {
                if (response.status !== 200) return res2err(response, cb);
                return response.json();
            }).then((hook) => {
                return cb(null, hook);
            }).catch((err) => {
                return cb(err);
            });
        },
        create: function(webhook, cb) {
            if (!webhook) return cb(new Error('Webhook required'));

            fetch(`${window.location.protocol}//${window.location.host}/api/webhooks`, {
                method: 'POST',
                credentials: 'same-origin',
                headers: new Headers({
                    'Content-Type': 'application/json'
                }),
                body: JSON.stringify(webhook)
            }).then((response) => {
                if (response.status !== 200) return res2err(response, cb);
                return response.json();
            }).then((hook) => {
                return cb(null, hook);
            }).catch((err) => {
                return cb(err);
            });
        },
        update: function(id, webhook, cb) {
            if (!id) return cb(new Error('Webhook ID required'));
            if (!webhook) return cb(new Error('Webhook required'));

            fetch(`${window.location.protocol}//${window.location.host}/api/webhooks/${id}`, {
                method: 'POST',
                credentials: 'same-origin',
                headers: new Headers({
                    'Content-Type': 'application/json'
                }),
                body: JSON.stringify(webhook)
            }).then((response) => {
                if (response.status !== 200) return res2err(response, cb);
                return response.json();
            }).then((hook) => {
                return cb(null, hook);
            }).catch((err) => {
                return cb(err);
            });
        }
    }
}
