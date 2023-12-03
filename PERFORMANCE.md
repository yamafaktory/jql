| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 20.4 ± 0.5 | 19.7 | 29.9 | 7.02 ± 0.42 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.9 ± 0.2 | 2.5 | 4.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 20.3 ± 0.4 | 19.4 | 22.3 | 7.06 ± 0.39 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.9 ± 0.1 | 2.5 | 3.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 20.2 ± 0.5 | 19.3 | 30.1 | 6.53 ± 0.41 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 3.1 ± 0.2 | 2.6 | 3.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 122.1 ± 5.7 | 115.5 | 193.5 | 6.39 ± 0.34 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 19.1 ± 0.5 | 17.5 | 22.5 | 1.00 |

