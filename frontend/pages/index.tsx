import { GetStaticProps } from "next";

import getTopics from "@lib/api/getTopics";
import Home from "@lib/components/Home";
import revalidateTime from "@lib/revalidateTime";

export default Home;

export const getStaticProps: GetStaticProps = async (_context) => {
  const topics = await getTopics();
  return {
    props: { topics },
    revalidate: revalidateTime,
  };
};
