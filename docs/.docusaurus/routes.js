import React from 'react';
import ComponentCreator from '@docusaurus/ComponentCreator';

export default [
  {
    path: '/pollen/__docusaurus/debug',
    component: ComponentCreator('/pollen/__docusaurus/debug', '6a4'),
    exact: true
  },
  {
    path: '/pollen/__docusaurus/debug/config',
    component: ComponentCreator('/pollen/__docusaurus/debug/config', 'e27'),
    exact: true
  },
  {
    path: '/pollen/__docusaurus/debug/content',
    component: ComponentCreator('/pollen/__docusaurus/debug/content', 'bba'),
    exact: true
  },
  {
    path: '/pollen/__docusaurus/debug/globalData',
    component: ComponentCreator('/pollen/__docusaurus/debug/globalData', '0b0'),
    exact: true
  },
  {
    path: '/pollen/__docusaurus/debug/metadata',
    component: ComponentCreator('/pollen/__docusaurus/debug/metadata', 'fc3'),
    exact: true
  },
  {
    path: '/pollen/__docusaurus/debug/registry',
    component: ComponentCreator('/pollen/__docusaurus/debug/registry', '313'),
    exact: true
  },
  {
    path: '/pollen/__docusaurus/debug/routes',
    component: ComponentCreator('/pollen/__docusaurus/debug/routes', '9bd'),
    exact: true
  },
  {
    path: '/pollen/markdown-page',
    component: ComponentCreator('/pollen/markdown-page', '1e9'),
    exact: true
  },
  {
    path: '/pollen/docs',
    component: ComponentCreator('/pollen/docs', 'e2a'),
    routes: [
      {
        path: '/pollen/docs',
        component: ComponentCreator('/pollen/docs', 'e57'),
        routes: [
          {
            path: '/pollen/docs',
            component: ComponentCreator('/pollen/docs', '743'),
            routes: [
              {
                path: '/pollen/docs/',
                component: ComponentCreator('/pollen/docs/', '286'),
                exact: true
              },
              {
                path: '/pollen/docs/commands',
                component: ComponentCreator('/pollen/docs/commands', 'a9c'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pollen/docs/configuration',
                component: ComponentCreator('/pollen/docs/configuration', '998'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pollen/docs/dependency_management',
                component: ComponentCreator('/pollen/docs/dependency_management', 'f85'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pollen/docs/directory_structure',
                component: ComponentCreator('/pollen/docs/directory_structure', '808'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pollen/docs/environment_variables',
                component: ComponentCreator('/pollen/docs/environment_variables', '66c'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pollen/docs/git_integration',
                component: ComponentCreator('/pollen/docs/git_integration', 'a33'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pollen/docs/installation',
                component: ComponentCreator('/pollen/docs/installation', 'd66'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pollen/docs/intro',
                component: ComponentCreator('/pollen/docs/intro', 'bfd'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/pollen/docs/usage',
                component: ComponentCreator('/pollen/docs/usage', '759'),
                exact: true,
                sidebar: "tutorialSidebar"
              }
            ]
          }
        ]
      }
    ]
  },
  {
    path: '/pollen/',
    component: ComponentCreator('/pollen/', 'f33'),
    exact: true
  },
  {
    path: '*',
    component: ComponentCreator('*'),
  },
];
