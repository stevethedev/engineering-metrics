import path from "path";
import HtmlWebpackPlugin from "html-webpack-plugin";
import MiniCssExtractPlugin from "mini-css-extract-plugin";
import WorkboxWebpackPlugin from "workbox-webpack-plugin";
import { Configuration, EnvironmentPlugin } from "webpack";
import "webpack-dev-server";

const isProduction = process.env.NODE_ENV === "production";

const stylesHandler = isProduction
  ? MiniCssExtractPlugin.loader
  : "style-loader";

const cssLoader = {
  loader: "css-loader",
  options: {
    importLoaders: 1,
    modules: {
      mode: "local",
    },
  },
};

const envData = Object.keys(process.env).reduce<
  Record<string, string | null | undefined>
>((acc, key) => {
  acc[key] = null;
  return acc;
}, {});

const config: Configuration = {
  entry: "./src/index.tsx",
  output: {
    path: path.resolve(__dirname, "dist"),
  },
  devServer: {
    open: true,
    host: "localhost",
  },
  plugins: [
    new EnvironmentPlugin({
      ...envData,
      API_BASE_URL: null,
      API_AUTH_URL: null,
      API_AUTH_LOGIN_URL: null,
      API_AUTH_LOGOUT_URL: null,
      API_AUTH_WHOAMI_URL: null,
      API_TOKEN_KEY: null,
      API_TOKEN_EXPIRES_KEY: null,
      API_REFRESH_KEY: null,
      API_REFRESH_EXPIRES_KEY: null,
      API_AUTH_REFRESH_URL: null,
    }),

    new HtmlWebpackPlugin({
      template: "index.html",
    }),
  ],
  module: {
    rules: [
      {
        test: /\.(ts|tsx)$/i,
        loader: "ts-loader",
        exclude: ["/node_modules/"],
      },
      {
        test: /\.s[ac]ss$/i,
        use: [stylesHandler, cssLoader, "postcss-loader", "sass-loader"],
      },
      {
        test: /\.css$/i,
        use: [stylesHandler, cssLoader, "postcss-loader"],
      },
      {
        test: /\.(eot|svg|ttf|woff|woff2|png|jpg|gif)$/i,
        type: "asset",
      },

      // Add your rules for custom modules here
      // Learn more about loaders from https://webpack.js.org/loaders/
    ],
  },
  resolve: {
    extensions: [".tsx", ".ts", ".jsx", ".js", "..."],
  },
};

export default () => {
  if (isProduction) {
    process.stdout.write("Building for production...\n");
    config.mode = "production";
    config.plugins?.push(new MiniCssExtractPlugin());
    config.plugins?.push(new WorkboxWebpackPlugin.GenerateSW());
  } else {
    process.stdout.write("Building for development...\n");
    config.mode = "development";
  }
  return config;
};
