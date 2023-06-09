import type { CodegenConfig } from '@graphql-codegen/cli'
 
const config: CodegenConfig = {
  schema: 'http://localhost:8000/',
  documents: './src/**/*.gql',
  generates: {
    './src/graphql/generated.ts': {
      plugins: ['typescript', 'typescript-operations', 'graphql-codegen-svelte-apollo'],
      config: {
        clientPath: './client',
        asyncQuery: true
      }
    }
  }
}
export default config