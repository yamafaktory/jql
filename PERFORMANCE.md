| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 2.6 ± 0.1 | 2.5 | 3.9 | 1.15 ± 0.11 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.3 ± 0.2 | 2.1 | 8.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 2.5 ± 0.1 | 2.4 | 3.7 | 1.13 ± 0.13 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.2 ± 0.2 | 2.0 | 5.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 2.6 ± 0.1 | 2.4 | 4.4 | 1.16 ± 0.11 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.2 ± 0.2 | 2.1 | 5.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 91.2 ± 4.3 | 86.9 | 108.4 | 4.99 ± 0.32 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 18.3 ± 0.8 | 17.3 | 31.1 | 1.00 |

