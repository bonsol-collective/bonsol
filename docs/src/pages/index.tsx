import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';
import Heading from '@theme/Heading';
import React from 'react';
import styles from './index.module.css';

function DisclaimerBanner() {
  return (
    <div className={styles.disclaimerBanner}>
      <div className="container">
        <div className="alert alert--warning margin-bottom--md">
          <strong>📢 Important Notice:</strong> This documentation site is being migrated to our new platform. 
          For the most up-to-date documentation, please visit{' '}
          <Link href="https://docs.bonsol.org" className="alert__link">
            docs.bonsol.org
          </Link>
          {' '}instead.
        </div>
      </div>
    </div>
  );
}

function HomepageHeader() {
  const {siteConfig} = useDocusaurusContext();
  return (
    <header className={clsx('hero hero--primary', styles.heroBanner)}>
      <div className="container">
        <Heading as="h1" className="hero__title">
          {siteConfig.title}
        </Heading>
        <p className="hero__subtitle">{siteConfig.tagline}</p>
        <div className={styles.buttons}>
          <Link
            className="button button--secondary button--lg"
            to="/docs/tutorials/a-taste-of-bonsol">
            Bonsol Tutorial - 1hr ⏱️
          </Link>
        </div>
      </div>
    </header>
  );
}

export default function Home(): JSX.Element {
  const {siteConfig} = useDocusaurusContext();
  return (
    <Layout
      title={`${siteConfig.title} Docs`}
      description="Bonsol zk plubing on solana">
      <DisclaimerBanner />
      <HomepageHeader />
      <main>
        <HomepageFeatures />
      </main>
    </Layout>
  );
}
