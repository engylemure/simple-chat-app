import { AsyncRooms } from "../../graphql/generated";
import { getUser } from "../../lib/state/user";
import type { PageServerLoad } from "./$types";

export const load = (
  async (params) => {
    const rooms = await AsyncRooms({});
    return { user: await getUser(params), rooms }
  }
) satisfies PageServerLoad;
