| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 42.7 ± 1.7 | 39.9 | 52.7 | 18.54 ± 4.86 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.3 ± 0.6 | 1.7 | 9.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 42.4 ± 2.0 | 39.0 | 54.6 | 19.51 ± 4.17 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.2 ± 0.5 | 1.5 | 9.1 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 42.3 ± 2.1 | 39.2 | 55.7 | 19.09 ± 4.06 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.2 ± 0.5 | 1.6 | 10.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 193.9 ± 5.9 | 183.7 | 237.6 | 1.34 ± 0.07 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 145.1 ± 5.7 | 135.5 | 185.7 | 1.00 |

