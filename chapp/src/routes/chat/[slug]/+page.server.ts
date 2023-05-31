import client from '../../../graphql/client';
import { AsyncRoom, RoomDoc } from "../../../graphql/generated";
import { getUser } from "../../../lib/state/user";
import type { PageServerLoad } from "./$types";

export const load = (
  async ({ params, cookies }) => {
    const lastAmount = 20;
    const user = await getUser({ cookies });
    const room = await AsyncRoom({ variables: { id: params.slug, last: lastAmount }, fetchPolicy: 'network-only'  });
    if (room) {
      client.cache.writeQuery({
        query: RoomDoc,
        variables: { id: params.slug, last: lastAmount },
        data: room.data
      });
    }
    return { user, room: room.data?.room, roomId: params.slug, lastAmount }
  }
) satisfies PageServerLoad;