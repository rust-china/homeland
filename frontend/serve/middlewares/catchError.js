module.exports = async (ctx, next) => {
	try {
		await next();
	} catch (error) {
		return ctx.body = error;
	}
}