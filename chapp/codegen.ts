import type { CodegenConfig } from '@graphql-codegen/cli'
 
const config: CodegenConfig = {
  schema: 'http://localhost:8000/',
  documents: './src/**/*.gql',
  generates: {
    './src/graphql/generated.ts': {
      plugins: ['typescript', 'typescript-operations', 'typed-document-node', '@kitql/graphql-codegen'],
      config: {
        useTypeImports: true
      }
    }
  }
}
export default config