| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 42.1 ± 3.6 | 35.4 | 61.3 | 20.75 ± 5.70 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.0 ± 0.5 | 1.3 | 6.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 41.6 ± 3.4 | 35.4 | 59.8 | 19.96 ± 5.64 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.1 ± 0.6 | 1.3 | 7.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 41.9 ± 3.7 | 34.9 | 61.6 | 20.62 ± 6.22 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.0 ± 0.6 | 1.2 | 8.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 182.5 ± 12.5 | 159.3 | 237.6 | 1.27 ± 0.13 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 143.2 ± 10.7 | 123.8 | 187.3 | 1.00 |

