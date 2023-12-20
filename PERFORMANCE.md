| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 19.9 ± 0.3 | 19.3 | 21.9 | 7.16 ± 0.41 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.8 ± 0.2 | 2.4 | 4.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 20.1 ± 0.3 | 19.4 | 21.8 | 7.18 ± 0.39 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.8 ± 0.1 | 2.5 | 3.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 19.7 ± 0.3 | 19.1 | 23.4 | 6.99 ± 0.44 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.8 ± 0.2 | 2.4 | 3.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 122.0 ± 5.2 | 116.1 | 184.9 | 6.50 ± 0.33 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 18.8 ± 0.5 | 17.1 | 22.1 | 1.00 |

