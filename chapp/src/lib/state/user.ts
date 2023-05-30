import { AsyncMe, CreateUser } from "../../graphql/generated";
import type { Cookies } from "@sveltejs/kit";

export async function getUser(params: { cookies: Cookies }) {
    const { cookies } = params;
    let userId = cookies.get('user_id');
    const user = (await AsyncMe({ context: { headers: { user_id: userId } }, fetchPolicy: 'network-only' }));
    if (!user.data?.me) {
      cookies.delete('user_id');
      const response = await CreateUser({});
      userId = response.data?.createUser.id; 
    }
    if (userId) {
      cookies.set('user_id', userId);
    }
    return (await AsyncMe({ context: { headers: { user_id: userId } }, fetchPolicy: 'network-only' })).data?.me
}