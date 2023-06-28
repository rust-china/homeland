module.exports = {
  plugins: {
    // pnpm i tailwindcss -D
    'tailwindcss': {},
    // pnpm i autoprefixer -D
    autoprefixer: {},
    // pnpm i cssnano -D
    ...(process.env.NODE_ENV === 'production' ? { cssnano: {} } : {})
  },
};