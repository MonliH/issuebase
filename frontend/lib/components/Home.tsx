import { useRouter } from "next/router";
import DefaultErrorPage from "next/error";

import styles from "@styles/Home.module.css";
import { ActiveTopic, TopicIssues } from "@lib/api/getTopicIssues";
import { Projects } from "@lib/api/getTopics";

import Issues from "@lib/components/Issues";
import Sidebar from "@lib/components/Sidebar";
import Loading from "@lib/components/Loading";
import Layout from "@lib/components/Layout";

export default function Home({
  active,
  topics,
  issues,
}: {
  active?: ActiveTopic;
  topics: Projects;
  issues?: TopicIssues | null;
}) {
  const router = useRouter();

  if (router.isFallback) {
    return (
      <Layout>
        <div className={styles.loadingContainer}>
          <Loading className={styles.loading} />
          Loading...
        </div>
      </Layout>
    );
  }

  if (issues === null) {
    return (
      <Layout>
        <DefaultErrorPage statusCode={404} />
      </Layout>
    );
  }

  return (
    <Layout>
      <div className={styles.container}>
        <main className={styles.mainLayout}>
          <Sidebar topics={topics} active={active} />
          <div className={styles.scrollable}>
            <Issues
              issues={issues}
              name={
                active
                  ? topics[active.language].groups[active.categoryIdx].name
                  : undefined
              }
            />
          </div>
        </main>
      </div>
    </Layout>
  );
}
