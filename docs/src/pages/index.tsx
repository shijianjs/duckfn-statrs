import {createElement} from 'react';
import type {ReactNode} from 'react';
import Link from '@docusaurus/Link';
import Translate, {translate} from '@docusaurus/Translate';
import useBaseUrl from '@docusaurus/useBaseUrl';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import CodeBlock from '@theme/CodeBlock';
import Heading from '@theme/Heading';
import Layout from '@theme/Layout';
import {
  DfkFeatures,
  DfkHero,
  DfkNextSteps,
  registerDfkElements,
  type FeatureItem,
  type HeroAction,
  type HeroBadge,
  type HeroLink,
  type NextStepItem,
} from 'duckfn-docs-kit';

import styles from './index.module.css';

// Defining the `dfk-*` custom elements is a one-time side effect (it also
// registers the official `<iconify-icon>` element). Idempotent, and a no-op
// during Docusaurus' Node prerender pass.
registerDfkElements();

/**
 * The landing page: hero, features, a Rust/SQL showcase and the "where next"
 * cards.
 *
 * The hero, the feature grid and the next-step cards are `dfk-*` web components
 * from duckfn-docs-kit, so the whole landing layout is reusable by other
 * extension docs sites. A custom element cannot render React's `<Translate>`,
 * so the copy is resolved with the imperative `translate()` API into plain
 * strings for the active locale and handed to the components through their own
 * setters; the strings still live in `i18n/zh-Hans/code.json` under the same
 * `homepage.*` keys. The code showcase stays here because it needs the theme's
 * `CodeBlock`.
 *
 * Known trade-off (accepted): the `dfk-*` sections render client-side, so their
 * prerendered HTML is empty until hydration — the same behaviour as the
 * `<iconify-icon>` glyphs.
 */

type DfkTag = 'dfk-hero' | 'dfk-features' | 'dfk-next-steps';

/**
 * Mounts a `dfk-*` element and hands its content over through the component's
 * own setters.
 *
 * React's SSR/hydration path only reconciles string/number props onto custom
 * elements — an object payload is never serialised into the prerendered HTML,
 * so hydration would leave the component empty. A callback ref is the reliable
 * channel: React calls it with the live node after mount, where the setters are
 * invoked. The components are retained-mode, so each setter only mutates the
 * nodes it owns; nothing is re-rendered.
 */
function dfk<TContent>(
  tag: DfkTag,
  mount: (node: HTMLElement, content: TContent) => void,
  content: TContent,
): ReactNode {
  return createElement(tag, {
    ref: (node: HTMLElement | null) => {
      if (node) {
        mount(node, content);
      }
    },
  });
}

interface HeroContent {
  logoSrc: string;
  title: string;
  tagline: string;
  primary: HeroLink;
  secondary: HeroAction;
  badges: HeroBadge[];
}

function mountHero(node: HTMLElement, content: HeroContent): void {
  const hero = node as DfkHero;
  hero.setLogo(content.logoSrc);
  hero.setTitle(content.title);
  hero.setTagline(content.tagline);
  hero.setPrimaryAction(content.primary);
  hero.setSecondaryAction(content.secondary);
  hero.setBadges(content.badges);
}

interface FeaturesContent {
  sectionTitle: string;
  items: FeatureItem[];
}

function mountFeatures(node: HTMLElement, content: FeaturesContent): void {
  const features = node as DfkFeatures;
  features.setSectionTitle(content.sectionTitle);
  features.setFeatures(content.items);
}

interface NextStepsContent {
  sectionTitle: string;
  items: NextStepItem[];
}

function mountNextSteps(node: HTMLElement, content: NextStepsContent): void {
  const steps = node as DfkNextSteps;
  steps.setSectionTitle(content.sectionTitle);
  steps.setSteps(content.items);
}

/**
 * Kept out of the JSX below on purpose: a template literal written inline would
 * carry the JSX indentation into the rendered code block.
 */
const SQL_SAMPLE = `-- Install once, then LOAD in every session:
INSTALL duckfn_statrs FROM community;
LOAD duckfn_statrs;

-- Aggregates: one column in, one value out per group:
SELECT g, sr_mean(x) AS mean, sr_std_dev(x) AS sd
FROM my_table GROUP BY g;

-- Scalars: distribution functions evaluated row by row:
SELECT sr_normal_pdf(0.0, 0.0, 1.0);    -- 0.3989...
SELECT sr_gamma_cdf(2.0, 3.0, 2.0);    -- 0.4564...

-- NULL semantics: statrs cannot define it → SQL NULL
SELECT sr_variance(x) FROM (VALUES (1.0)) t(x);  -- NULL (single value)`;

/**
 * The shields.io badges ask for `style=flat`, which is the rounded style; the
 * default `flat-square` draws square corners and would clash with the language
 * badges below, which are rounded too. The row has to look like one set, so the
 * shape is decided at the source rather than patched with CSS.
 *
 * 徽章：`<owner>/<repo>` 还没填时（`REPO_URL` 仍是占位符）只显示不依赖仓库地址的三枚，
 * 免得首页挂着一排坏图。
 */
function badges(repoUrl: string): HeroBadge[] {
  const repoSlug = repoUrl.replace(/^https:\/\/github\.com\//, '');
  const hasRepo = !repoSlug.includes('<');

  return [
    ...(hasRepo
      ? [
          {
            href: `${repoUrl}/releases`,
            src: `https://img.shields.io/github/v/release/${repoSlug}?style=flat`,
            alt: 'Latest release',
          },
        ]
      : []),
    {
      href: 'https://github.com/shijianjs/duckfn-statrs/blob/main/LICENSE',
      src: 'https://img.shields.io/badge/license-MIT-14459b.svg?style=flat',
      alt: 'MIT license',
    },
    {
      href: 'https://rust-lang.org',
      src: 'https://img.shields.io/badge/Rust-1.89%2B-14459b.svg?style=flat',
      alt: 'Rust 1.89 or newer',
    },
    {
      href: 'https://duckdb.org',
      src: 'https://img.shields.io/badge/DuckDB-1.3%2B-14459b.svg?style=flat',
      alt: 'DuckDB 1.3 or newer',
    },
  ];
}

function heroContent(
  logoSrc: string,
  introHref: string,
  repoUrl: string,
  title: string,
  tagline: string,
): HeroContent {
  return {
    logoSrc,
    title,
    tagline,
    primary: {
      label: translate({id: 'homepage.getStarted', message: 'Get started'}),
      href: introHref,
    },
    secondary: {
      label: translate({
        id: 'homepage.github',
        description: 'Home page button linking to the repository',
        message: 'GitHub',
      }),
      href: repoUrl,
      icon: 'simple-icons:github',
    },
    badges: badges(repoUrl),
  };
}

function featuresContent(): FeaturesContent {
  return {
    sectionTitle: translate({
      id: 'homepage.features.title',
      description: 'Home page section title above the feature cards',
      message: 'What you get',
    }),
    items: [
      {
        icon: 'lucide:chart-line',
        title: translate({
          id: 'homepage.features.stats.title',
          description: 'Home page feature card title',
          message: 'Summary statistics as aggregates',
        }),
        details: translate({
          id: 'homepage.features.stats.details',
          description: 'Home page feature card description',
          message:
            'Mean, median, variance, covariance, quantiles, ranks and more — all native SQL aggregates that work with GROUP BY.',
        }),
      },
      {
        icon: 'lucide:sigma',
        title: translate({
          id: 'homepage.features.dist.title',
          description: 'Home page feature card title',
          message: '27 probability distributions',
        }),
        details: translate({
          id: 'homepage.features.dist.details',
          description: 'Home page feature card description',
          message:
            'pdf, cdf, survival function, quantile, and log-density for every distribution — continuous, discrete, and multivariate.',
        }),
      },
      {
        icon: 'lucide:dice-5',
        title: translate({
          id: 'homepage.features.sample.title',
          description: 'Home page feature card title',
          message: 'Random sampling & signal generation',
        }),
        details: translate({
          id: 'homepage.features.sample.details',
          description: 'Home page feature card description',
          message:
            'Draw k samples from any distribution into a LIST; generate sinusoidal, square, triangle, and sawtooth waveforms.',
        }),
      },
      {
        icon: 'lucide-flask-conical',
        title: translate({
          id: 'homepage.features.tests.title',
          description: 'Home page feature card title',
          message: 'Hypothesis testing',
        }),
        details: translate({
          id: 'homepage.features.tests.details',
          description: 'Home page feature card description',
          message:
            't-test, chi-square, ANOVA, Mann-Whitney U, Kolmogorov-Smirnov, Anderson-Darling, Fisher’s exact — all in SQL.',
        }),
      },
      {
        icon: 'lucide:function-square',
        title: translate({
          id: 'homepage.features.func.title',
          description: 'Home page feature card title',
          message: 'Special functions & constants',
        }),
        details: translate({
          id: 'homepage.features.func.details',
          description: 'Home page feature card description',
          message:
            'Gamma, Beta, erf, digamma, factorials, binomial coefficients, harmonic numbers, and mathematical constants.',
        }),
      },
      {
        icon: 'lucide:shield-check',
        title: translate({
          id: 'homepage.features.null.title',
          description: 'Home page feature card title',
          message: 'Predictable NULL semantics',
        }),
        details: translate({
          id: 'homepage.features.null.details',
          description: 'Home page feature card description',
          message:
            'Undefined results become SQL NULL, invalid parameters raise clear errors, NULL inputs never produce a number — consistent across all 252 functions.',
        }),
      },
    ],
  };
}

function nextStepsContent(
  hrefs: readonly [string, string, string, string],
): NextStepsContent {
  const [quickStart, functions, structure, release] = hrefs;
  return {
    sectionTitle: translate({
      id: 'homepage.next.title',
      description: 'Home page section title above the link cards',
      message: 'Where to go next',
    }),
    items: [
      {
        href: quickStart,
        title: translate({
          id: 'homepage.next.quickStart.title',
          description: 'Home page link card title',
          message: 'Install and load',
        }),
        details: translate({
          id: 'homepage.next.quickStart.details',
          description: 'Home page link card description',
          message: 'Get the extension running in DuckDB and try your first queries.',
        }),
      },
      {
        href: functions,
        title: translate({
          id: 'homepage.next.functions.title',
          description: 'Home page link card title',
          message: 'Function reference',
        }),
        details: translate({
          id: 'homepage.next.functions.details',
          description: 'Home page link card description',
          message:
            'Browse all 252 functions organized by category: statistics, distributions, special functions, sampling, and tests.',
        }),
      },
      {
        href: structure,
        title: translate({
          id: 'homepage.next.structure.title',
          description: 'Home page link card title',
          message: 'Development guide',
        }),
        details: translate({
          id: 'homepage.next.structure.details',
          description: 'Home page link card description',
          message: 'Build from source, write new functions, test, and release.',
        }),
      },
      {
        href: release,
        title: translate({
          id: 'homepage.next.release.title',
          description: 'Home page link card title',
          message: 'Build and release',
        }),
        details: translate({
          id: 'homepage.next.release.details',
          description: 'Home page link card description',
          message: 'The build paths, the release flow, and the wasm target.',
        }),
      },
    ],
  };
}

function CodeShowcase(): ReactNode {
  return (
    <section className={styles.sectionTint}>
      <div className={styles.sectionInner}>
        <Heading as="h2" className={styles.sectionTitle}>
          <Translate
            id="homepage.showcase.title"
            description="Home page section title above the SQL code block">
            Statistics in plain SQL
          </Translate>
        </Heading>
        <p className={styles.sectionLead}>
          <Translate
            id="homepage.showcase.lead"
            description="Home page paragraph introducing the SQL code block">
            252 functions backed by the Rust statrs library, callable directly
            from your DuckDB queries. Aggregates for summary statistics, scalars
            for distributions and special functions.
          </Translate>
        </p>
        <div className={styles.codeGrid}>
          <div className={styles.codeColumn}>
            <CodeBlock language="sql" title="DuckDB SQL">
              {SQL_SAMPLE}
            </CodeBlock>
          </div>
        </div>
        <p className={styles.showcaseLinkRow}>
          <Link className={styles.showcaseLink} to="/docs/guide/functions/overview">
            <Translate
              id="homepage.showcase.link"
              description="Home page link to the functions guide">
              Browse all functions
            </Translate>
            {createElement('iconify-icon', {
              icon: 'lucide:arrow-right',
              className: styles.showcaseLinkArrow,
              'aria-hidden': 'true',
            })}
          </Link>
        </p>
      </div>
    </section>
  );
}

export default function Home(): ReactNode {
  const {siteConfig} = useDocusaurusContext();
  const repoUrl = siteConfig.customFields?.repoUrl as string;
  // Not a hard-coded "/img/...": the site is published under /<repo>/ on GitHub
  // Pages, and only useBaseUrl adds that prefix. The dfk-* components render
  // plain anchors, so every internal href is resolved here before it is passed
  // in.
  const logoUrl = useBaseUrl('img/logo.svg');
  const introUrl = useBaseUrl('/docs/intro');
  const nextHrefs = [
    useBaseUrl('/docs/getting-started/quick-start'),
    useBaseUrl('/docs/guide/functions/overview'),
    useBaseUrl('/docs/getting-started/project-structure'),
    useBaseUrl('/docs/build-and-release'),
  ] as const;

  return (
    <Layout
      title={siteConfig.title}
      description="Documentation for duckfn_statrs: 252 statistical functions for DuckDB SQL, from the Rust statrs crate.">
      {/* Layout renders no <main> of its own: this is the page's only one. */}
      <main>
        {dfk(
          'dfk-hero',
          mountHero,
          heroContent(
            logoUrl,
            introUrl,
            repoUrl,
            siteConfig.title,
            translate({id: 'homepage.tagline', message: siteConfig.tagline}),
          ),
        )}
        {dfk('dfk-features', mountFeatures, featuresContent())}
        <CodeShowcase />
        {dfk('dfk-next-steps', mountNextSteps, nextStepsContent(nextHrefs))}
      </main>
    </Layout>
  );
}