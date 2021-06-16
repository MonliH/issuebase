import { ReactNode } from "react";
import Head from "next/head";

export default function Layout({ children }: { children?: ReactNode }) {
  return (
    <>
      <Head>
        <title>issuebase</title>
        <meta
          name="description"
          content="Fetch good first issues on lots of curated repositories on github"
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      {children}
    </>
  );
}
