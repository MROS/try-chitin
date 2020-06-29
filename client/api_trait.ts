export type Result<T, E> = {
    'Ok': T
} | {
    'Err': E
};
export type Article = { author_id: number; title: string; content: string };
export type User = { name: string; sentence: string };
export abstract class RootQueryFetcher {
    abstract fetchResult(query: Object): Promise<string>;
    async askArticles(count: number): Promise<Result<Array<Article>, string>> {
        return JSON.parse(await this.fetchResult({ "AskArticles": { count } }));
    }
    async postArticle(article: Article): Promise<Result<null, string>> {
        return JSON.parse(await this.fetchResult({ "PostArticle": { article } }));
    }
    async createUser(user: User): Promise<Result<number, string>> {
        return JSON.parse(await this.fetchResult({ "CreateUser": { user } }));
    }
    async askUserArticles(count: number, user_id: number): Promise<Result<Array<Article>, string>> {
        return JSON.parse(await this.fetchResult({ "User": { "AskUserArticles": { count, user_id } } }));
    }
    async askUserDetail(user_id: number): Promise<Result<User, string>> {
        return JSON.parse(await this.fetchResult({ "User": { "UserDetail": { "AskUserDetail": { user_id } } } }));
    }
}
