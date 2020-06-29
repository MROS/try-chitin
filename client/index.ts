import * as _vorpal from 'vorpal';
import * as api from './api';

const vorpal = new _vorpal();

const api_fetcher = new api.ApiFetcher();

function report(promise: Promise<any>): Promise<void> {
    return promise
        .then(resp => {
            this.log(resp);
        })
        .catch(err => {
            this.log(err);
        });
}

vorpal
    .command('articles <number>')
    .action(function (args) {
        return report.bind(this)(api_fetcher.askArticles(args.number));
    });

vorpal
    .command('user_articles <user_id> <number>')
    .action(function (args) {
        return report.bind(this)(api_fetcher.askUserArticles(args.number, args.user_id));
    });

vorpal
    .command('user_detail <user_id>')
    .action(function (args) {
        return report.bind(this)(api_fetcher.askUserDetail(args.user_id));
    });

vorpal
    .command('create_user <name> <sentence>')
    .action(function (args) {
        return report.bind(this)(api_fetcher.createUser({ name: args.name, sentence: args.sentence }));
    });

vorpal
    .command('post <author_id> <title> <content>')
    .action(function (args) {
        return report.bind(this)(api_fetcher.postArticle({
            author_id: args.author_id,
            title: args.title,
            content: args.content
        }));
    });

vorpal
    .delimiter('$')
    .show();
