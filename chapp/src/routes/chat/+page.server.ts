import { KQL_CreateUser, KQL_Me } from "../../graphql/generated";
import type { PageServerLoad } from "../sverdle/$types";

export const load = (
  async ({ cookies, fetch }) => {
    let userId = cookies.get('user_id');
    const customFetch: typeof fetch = (input, init) => {
      const headers = new Headers(init?.headers);
      if (userId) {
        headers.append('user_id', userId)
      }
      return fetch(input, { ...init, headers })
    }
    const user = (await KQL_Me.query({ fetch: customFetch }));
    if (!user.data?.me) {
      const response = (await KQL_CreateUser.mutate({ fetch }));
      userId = response.data?.createUser.id; 
    }
    if (userId) {
      cookies.set('user_id', userId);
    }
    return { userId }
  }
) satisfies PageServerLoad;