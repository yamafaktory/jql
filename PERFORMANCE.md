| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 38.5 ± 2.2 | 34.7 | 51.9 | 14.40 ± 8.30 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.7 ± 1.5 | 1.8 | 29.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 39.1 ± 2.4 | 34.5 | 52.6 | 17.55 ± 6.25 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.2 ± 0.8 | 1.5 | 11.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 39.1 ± 2.4 | 34.6 | 56.1 | 16.91 ± 5.40 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.3 ± 0.7 | 1.6 | 14.0 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 195.3 ± 8.7 | 169.8 | 234.0 | 7.94 ± 1.29 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 24.6 ± 3.8 | 17.1 | 53.3 | 1.00 |

