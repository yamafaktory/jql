| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 37.0 ± 2.9 | 30.7 | 58.4 | 16.75 ± 4.10 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.2 ± 0.5 | 1.3 | 9.0 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 36.9 ± 2.7 | 30.7 | 55.3 | 17.92 ± 5.80 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.1 ± 0.6 | 1.1 | 14.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 36.5 ± 2.6 | 30.6 | 47.3 | 16.99 ± 4.29 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.1 ± 0.5 | 1.3 | 11.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 188.3 ± 8.4 | 166.8 | 220.6 | 1.35 ± 0.09 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 139.0 ± 7.0 | 122.5 | 173.3 | 1.00 |

