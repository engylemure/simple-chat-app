import { AsyncRoom, Room } from "../../../graphql/generated";
import { getUser } from "../../../lib/state/user";
import type { PageServerLoad } from "./$types";

export const load = (
  async ({ params, cookies }) => {
    const lastAmount = 20;
    const user = await getUser({ cookies });
    const room = await AsyncRoom({ variables: { id: params.slug, last: lastAmount }, fetchPolicy: 'network-only' });
    return { user, room: room.data?.room, lastAmount }
  }
) satisfies PageServerLoad;