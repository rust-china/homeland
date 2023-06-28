module.exports = async (ctx, next) => {
  // const token = ctx.cookies.get('authorization')
  await next()
}