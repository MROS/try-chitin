type Result<T, E> = {
    kind: 'ok',
    value: T
} | {
    kind: 'err',
    value: E
};

abstract class RootQueryFetcher {
    abstract fetchResult(query: Object): Promise<string>;
    async askArticles(count: number): Promise<Result<Array<Article>, string>> {
        return JSON.parse(await this.fetchResult({ "AskArticles": { count } }));
    }
    async postArticle(article: Article): Promise<Result<null, string>> {
        return JSON.parse(await this.fetchResult({ "PostArticle": { article } }));
    }
    async askUserArticles(count: number, user_id: number): Promise<Result<Array<Article>, string>> {
        return JSON.parse(await this.fetchResult({ "User": { "AskUserArticles": { count, user_id } } }));
    }
    async askUserDetail(user_id: number): Promise<Result<(User, string), string>> {
        return JSON.parse(await this.fetchResult({ "User": { "UserDetail": { "AskUserDetail": { user_id } } } }));
    }
}
