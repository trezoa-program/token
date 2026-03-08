module.exports = {
    extends: ['@trezoa/eslint-config-trezoa'],
    ignorePatterns: ['.eslintrc.cjs', 'tsup.config.ts', 'env-shim.ts'],
    parserOptions: {
        trezoa: 'tsconfig.json',
        tsconfigRootDir: __dirname,
        sourceType: 'module',
    },
    rules: {
        '@typescript-eslint/ban-types': 'off',
        '@typescript-eslint/sort-type-constituents': 'off',
        'prefer-destructuring': 'off',
        'simple-import-sort/imports': 'off',
        'sort-keys-fix/sort-keys-fix': 'off',
        'typescript-sort-keys/interface': 'off',
    },
};
