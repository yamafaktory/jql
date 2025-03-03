| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 2.7 ± 0.3 | 2.5 | 5.6 | 1.18 ± 0.14 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.3 ± 0.1 | 2.1 | 3.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 2.6 ± 0.2 | 2.5 | 6.3 | 1.10 ± 0.22 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.4 ± 0.4 | 2.1 | 7.1 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 2.7 ± 3.5 | 2.4 | 113.4 | 1.19 ± 1.56 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.3 ± 0.3 | 2.1 | 6.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 93.5 ± 4.9 | 87.7 | 106.9 | 4.98 ± 0.34 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 18.8 ± 0.8 | 17.6 | 29.0 | 1.00 |

