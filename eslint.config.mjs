import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { FlatCompat } from "@eslint/eslintrc";
import js from "@eslint/js";
import { defineConfig, globalIgnores } from "eslint/config";
import globals from "globals";

const __dirname = dirname(fileURLToPath(import.meta.url));

const compat = new FlatCompat({
	baseDirectory: __dirname,
	recommendedConfig: js.configs.recommended,
	allConfig: js.configs.all,
});

export default defineConfig([
	{
		languageOptions: {
			globals: {
				...globals.node,
			},
			ecmaVersion: "ESNext",
			parserOptions: {},
		},
		extends: compat.extends(
			"plugin:vue/vue3-essential",
			"eslint:recommended",
			"@vue/typescript/recommended",
			"plugin:prettier/recommended",
		),
		rules: {
			"no-console": process.env.NODE_ENV === "production" ? "warn" : "off",
			"no-debugger": process.env.NODE_ENV === "production" ? "warn" : "off",
			"@typescript-eslint/no-var-requires": "off",
			"@typescript-eslint/no-empty-function": "off",
		},
	},
	globalIgnores(["src/components/playground/", "src/vite-env.d.ts"]),
]);
