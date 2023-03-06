module.exports = (/** @type {import('plop').NodePlopAPI} */ plop) => {
  // create your generators here
  plop.setGenerator("block", {
    description: "React block component",
    prompts: [
      {
        type: "input",
        name: "name",
        message: "Component name",
      },
    ],
    actions: [
      {
        type: "add",
        path: "src/components/blocks/{{kebabCase name}}/index.ts",
        templateFile: ".plop-templates/block/index.ts.hbs",
      },
      {
        type: "add",
        path: "src/components/blocks/{{kebabCase name}}/{{kebabCase name}}.tsx",
        templateFile: ".plop-templates/block/block.tsx.hbs",
      },
      {
        type: "add",
        path: "src/components/blocks/{{kebabCase name}}/{{kebabCase name}}.module.scss",
        templateFile: ".plop-templates/block/block.module.scss.hbs",
      },
      {
        type: "add",
        path: "src/components/blocks/{{kebabCase name}}/{{kebabCase name}}.stories.tsx",
        templateFile: ".plop-templates/block/block.stories.tsx.hbs",
      },
    ],
  });

  plop.setGenerator("page", {
    description: "React page component",
    prompts: [
      {
        type: "input",
        name: "name",
        message: "Component name",
      },
    ],
    actions: [
      {
        type: "add",
        path: "src/components/pages/{{kebabCase name}}/index.ts",
        templateFile: ".plop-templates/page/index.ts.hbs",
      },
      {
        type: "add",
        path: "src/components/pages/{{kebabCase name}}/{{kebabCase name}}.tsx",
        templateFile: ".plop-templates/page/page.tsx.hbs",
      },
      {
        type: "add",
        path: "src/components/pages/{{kebabCase name}}/{{kebabCase name}}.stories.tsx",
        templateFile: ".plop-templates/page/page.stories.tsx.hbs",
      },
    ],
  });

  plop.setGenerator("container", {
    description: "React container component",
    prompts: [
      {
        type: "input",
        name: "name",
        message: "Component name",
      },
    ],
    actions: [
      {
        type: "add",
        path: "src/components/containers/{{kebabCase name}}/index.ts",
        templateFile: ".plop-templates/container/index.ts.hbs",
      },
      {
        type: "add",
        path: "src/components/containers/{{kebabCase name}}/{{kebabCase name}}.tsx",
        templateFile: ".plop-templates/container/container.tsx.hbs",
      },
      {
        type: "add",
        path: "src/components/containers/{{kebabCase name}}/{{kebabCase name}}.stories.tsx",
        templateFile: ".plop-templates/container/container.stories.tsx.hbs",
      },
    ],
  });

  plop.setGenerator("layout", {
    description: "React layout component",
    prompts: [
      {
        type: "input",
        name: "name",
        message: "Component name",
      },
    ],
    actions: [
      {
        type: "add",
        path: "src/components/layouts/{{kebabCase name}}/index.ts",
        templateFile: ".plop-templates/layout/index.ts.hbs",
      },
      {
        type: "add",
        path: "src/components/layouts/{{kebabCase name}}/{{kebabCase name}}.tsx",
        templateFile: ".plop-templates/layout/layout.tsx.hbs",
      },
      {
        type: "add",
        path: "src/components/layouts/{{kebabCase name}}/{{kebabCase name}}.stories.tsx",
        templateFile: ".plop-templates/layout/layout.stories.tsx.hbs",
      },
    ],
  });
};
