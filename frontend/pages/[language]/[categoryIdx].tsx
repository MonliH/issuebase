import { GetStaticPaths, GetStaticProps } from "next";

import Home from "@lib/components/Home";
import getTopics from "@lib/api/getTopics";
import revalidateTime from "@lib/revalidateTime";
import getTopicIssues from "@lib/api/getTopicIssues";

export default Home;

export const getStaticPaths: GetStaticPaths = async () => {
  const topics = await getTopics();
  const paths = [];
  for (const [language, groups] of Object.entries(topics)) {
    for (let i = 0; i < groups.groups.length; i++) {
      paths.push({ params: { language, categoryIdx: i.toString() } });
    }
  }
  return {
    paths,
    fallback: true,
  };
};

export const getStaticProps: GetStaticProps = async (context) => {
  const topics = await getTopics();
  const current = {
    language: context.params!.language as string,
    categoryIdx: Number.parseInt(context.params!.categoryIdx as string, 10),
  };
  const issues = await getTopicIssues(current);
  return {
    props: { topics, issues, current },
    revalidate: revalidateTime,
  };
};
