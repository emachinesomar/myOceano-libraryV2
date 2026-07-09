export const manifest = (() => {
function __memo(fn) {
	let value;
	return () => value ??= (value = fn());
}

return {
	appDir: "_app",
	appPath: "_app",
	assets: new Set(["favicon.svg"]),
	mimeTypes: {".svg":"image/svg+xml"},
	_: {
		client: {start:"_app/immutable/entry/start.R7JXcp4h.js",app:"_app/immutable/entry/app.C-1qJkss.js",imports:["_app/immutable/entry/start.R7JXcp4h.js","_app/immutable/chunks/BW5hWmra.js","_app/immutable/chunks/wtfjcbVx.js","_app/immutable/entry/app.C-1qJkss.js","_app/immutable/chunks/DYY2NPCk.js","_app/immutable/chunks/wtfjcbVx.js","_app/immutable/chunks/DctjwwZm.js","_app/immutable/chunks/Cr66bruD.js"],stylesheets:[],fonts:[],uses_env_dynamic_public:false},
		nodes: [
			__memo(() => import('./nodes/0.js')),
			__memo(() => import('./nodes/1.js')),
			__memo(() => import('./nodes/2.js'))
		],
		remotes: {
			
		},
		routes: [
			{
				id: "/",
				pattern: /^\/$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 2 },
				endpoint: null
			}
		],
		prerendered_routes: new Set([]),
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();
